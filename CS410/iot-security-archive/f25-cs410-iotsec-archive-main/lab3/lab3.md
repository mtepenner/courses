# __**Lab 3 Project: Building a LoRa Packet Sniffer**__

For Lab 3, I upgraded my LoRa project from Lab 2 into a full-fledged packet sniffer. My goal was to not only see packets on the OLED display but also to capture *all* LoRa traffic and stream it to my computer for analysis in Wireshark.

This involved two main challenges:
1.  Adding a live packet counter to my OLED display.
2.  Figuring out the PCAP format and streaming it over a second serial port.

## My Setup (Hardware & Software)

Here's the exact setup I used to get this working.

### Hardware
* **Board:** Adafruit Feather RP2040 RFM (with the LoRa radio)
* **Display:** Adafruit SH1107 OLED FeatherWing
* **USB Cable:** To connect to my computer.

### Software & Libraries
I'm running **CircuitPython 10.x** on my board. The most important part was getting the `lib` folder right.

My `lib` folder contains:
* `adafruit_rfm9x.mpy`
* `adafruit_displayio_sh1107.mpy`
* `i2cdisplaybus.mpy`
* `adafruit_display_text` (the folder)
* `keypad.mpy`
* `neopixel.mpy`

The new modules I needed for this lab (`time`, `usb_cdc`, `struct`) are thankfully built into CircuitPython, so I didn't need to add any new files to `lib` for them.

## The `boot.py` File: My "Data" Port

The first step for Task 3 was to create a secondary serial port. This was critical so I could send my clean PCAP data to Wireshark without mixing it up with all my `print()` messages and REPL errors.

I created a `boot.py` file on the root of my `CIRCUITPY` drive with just these two lines:

```python
import usb_cdc
# This enables the normal REPL (console=True) AND the new port (data=True)
usb_cdc.enable(console=True, data=True)
```
After saving this and rebooting my board, my computer saw two serial ports.


My Final File Structure
After adding boot.py, my CIRCUITPY drive looks like this:
```python
CIRCUITPY/
├── boot.py       <-- This was new for Lab 3
├── code.py       <-- My main code, heavily modified from Lab 2
└── lib/
    ├── adafruit_display_text/
    ├── adafruit_displayio_sh1107.mpy
    ├── adafruit_rfm9x.mpy
    ├── i2cdisplaybus.mpy
    ├── keypad.mpy
    └── neopixel.mpy
```
How I Upgraded My code.py
This is where I spent most of my time. I had to make two major upgrades to my old Lab 2 code.

## 1. Task 2: Adding the Packet Counter
This part was pretty easy.

### New Label: 
I added a new label.Label object called count_label and put it at the top of my display. I had to shift all my other labels down to make room.

### New Variable: 
I created a packet_count = 0 variable right before my while True: loop.

### Update in Loop: 
Inside the if packet is not None: block, I just added packet_count += 1 and then updated the label's text with count_label.text = f"Count: {packet_count}".

## 2. Task 3: Streaming PCAP Data
This was the real challenge. I had to research the PCAP format and figure out how to build it, byte by byte.

New Imports: First, I added three new imports to the top of my code.py:
```python
import time     # To get timestamps for each packet
import usb_cdc  # To write to my new "data" serial port
import struct   # This was the key! It packs my numbers into bytes
```
PCAP Global Header: Wireshark needs a "global header" at the very beginning of the stream, or it won't know what to do. I wrote a function write_pcap_global_header() that builds this. The struct.pack() function was essential here to turn my Python numbers into the exact C-style binary format PCAP expects.

I set the link-layer type (network) to 147, which is USER0—perfect for custom LoRa data.

I call this function only once, right before my main loop starts.

```python
if usb_cdc.data:
    write_pcap_global_header(usb_cdc.data)
    status_label.text = "PCAP Ready. Waiting..."
```
Here is what shows up on the display:
![An example image](20251112_150340_2.jpg "Test success")

### PCAP Packet Record: Next, I needed to send a header for every single packet. 

I wrote another function, write_pcap_packet_record(), to handle this.

Every time a packet comes in (if packet is not None:):

I immediately grab the timestamp using packet_timestamp = time.monotonic(). 

This gives me a high-resolution time since the board started.

My function splits this timestamp into seconds and microseconds.

It gets the packet length (len(packet_data)).

It uses struct.pack() again to create the binary packet record header.

Finally, it writes the record_header and then the raw packet_data to the usb_cdc.data port.

This is the code I call inside my if packet is not None: block:

```python
if usb_cdc.data:
    write_pcap_packet_record(usb_cdc.data, packet, packet_timestamp)
    print(f"Sent packet {packet_count} to PCAP")
```
## My Capture Workflow
Now, when I'm ready to sniff packets:


I plug in my Feather.

I connect my code editor (Thonny) to the first serial port (the "console") so I can see my print() messages.

I open Wireshark using my special cat command (since Wireshark couldn't find my ports).

I run this in my terminal: 

```bash
cat /dev/ttyACM1 | wireshark -k -i -
```

As soon as LoRa packets are received, they pop up live in Wireshark!
