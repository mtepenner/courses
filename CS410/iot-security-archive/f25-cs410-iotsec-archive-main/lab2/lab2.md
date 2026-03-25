# To start out, here is the code that I used to test if the Wifi capabilities of my FeatherWing work:

## SPDX-FileCopyrightText: 2019 ladyada for Adafruit Industries

## SPDX-License-Identifier: MIT

from os import getenv

import adafruit_connection_manager

import adafruit_requests

import board

import busio

from digitalio import DigitalInOut

from adafruit_esp32spi import adafruit_esp32spi

#Get wifi details and more from a settings.toml file

#tokens used by this Demo: CIRCUITPY_WIFI_SSID, CIRCUITPY_WIFI_PASSWORD

ssid = "*************"

password = "*********"

print("ESP32 SPI webclient test")

TEXT_URL = "http://wifitest.adafruit.com/testwifi/index.html"
JSON_URL = "http://wifitest.adafruit.com/testwifi/sample.json"


#If you are using a board with pre-defined ESP32 Pins:
#esp32_cs = DigitalInOut(board.ESP_CS)
#esp32_ready = DigitalInOut(board.ESP_BUSY)
#esp32_reset = DigitalInOut(board.ESP_RESET)

#If you have an AirLift Shield:
#esp32_cs = DigitalInOut(board.D10)
#esp32_ready = DigitalInOut(board.D7)
#esp32_reset = DigitalInOut(board.D5)

#If you have an AirLift Featherwing or ItsyBitsy Airlift:
esp32_cs = DigitalInOut(board.D13)
esp32_ready = DigitalInOut(board.D11)
esp32_reset = DigitalInOut(board.D12)

#If you have an externally connected ESP32:
#NOTE: You may need to change the pins to reflect your wiring
#esp32_cs = DigitalInOut(board.D9)
#esp32_ready = DigitalInOut(board.D10)
#esp32_reset = DigitalInOut(board.D5)

#Secondary (SCK1) SPI used to connect to WiFi board on Arduino Nano Connect RP2040
if "SCK1" in dir(board):
    spi = busio.SPI(board.SCK1, board.MOSI1, board.MISO1)
else:
    spi = busio.SPI(board.SCK, board.MOSI, board.MISO)
esp = adafruit_esp32spi.ESP_SPIcontrol(spi, esp32_cs, esp32_ready, esp32_reset)

pool = adafruit_connection_manager.get_radio_socketpool(esp)
ssl_context = adafruit_connection_manager.get_radio_ssl_context(esp)
requests = adafruit_requests.Session(pool, ssl_context)

if esp.status == adafruit_esp32spi.WL_IDLE_STATUS:
    print("ESP32 found and in idle mode")
print("Firmware vers.", esp.firmware_version)
print("MAC addr:", ":".join("%02X" % byte for byte in esp.MAC_address))

for ap in esp.scan_networks():
    print("\t%-23s RSSI: %d" % (ap.ssid, ap.rssi))

print("Connecting to AP...")
while not esp.is_connected:
    try:
        esp.connect_AP(ssid, password)
    except OSError as e:
        print("could not connect to AP, retrying: ", e)
        continue
print("Connected to", esp.ap_info.ssid, "\tRSSI:", esp.ap_info.rssi)
print("My IP address is", esp.ipv4_address)
print("IP lookup adafruit.com: %s" % esp.pretty_ip(esp.get_host_by_name("adafruit.com")))
print("Ping google.com: %d ms" % esp.ping("google.com"))

#esp._debug = True
print("Fetching text from", TEXT_URL)
r = requests.get(TEXT_URL)
print("-" * 40)
print(r.text)
print("-" * 40)
r.close()

print()
print("Fetching json from", JSON_URL)
r = requests.get(JSON_URL)
print("-" * 40)
print(r.json())
print("-" * 40)
r.close()

print("Done!")


I was indeed able to verify that my Airlift Featherwing does work when I downloaded the necessary libraries
and tested out my connection in class with my hotspot (Please don't make fun of its name).


![An example image](Screenshot_from_2025-10-27_13-42-54.png "Test success")

### Below is the code I pulled from Adafruit documentation along with the essential libraries to get the
### OLED display working:


## SPDX-FileCopyrightText: 2021 ladyada for Adafruit Industries
#
## SPDX-License-Identifier: Unlicense
"""
Author: Mark Roberts (mdroberts1243) from Adafruit code
This test will initialize the display using displayio and draw a solid white
background, a smaller black rectangle, miscellaneous stuff and some white text.

"""

import board
import displayio
import terminalio

#can try import bitmap_label below for alternative
from adafruit_display_text import label
from i2cdisplaybus import I2CDisplayBus

import adafruit_displayio_sh1107

displayio.release_displays()
#oled_reset = board.D9

#Use for I2C
i2c = board.I2C()  # uses board.SCL and board.SDA
#i2c = board.STEMMA_I2C()  # For using the built-in STEMMA QT connector on a microcontroller
display_bus = I2CDisplayBus(i2c, device_address=0x3C)

#SH1107 is vertically oriented 64x128
WIDTH = 128
HEIGHT = 64
BORDER = 2

display = adafruit_displayio_sh1107.SH1107(display_bus, width=WIDTH, height=HEIGHT)

#Make the display context
splash = displayio.Group()
display.root_group = splash

color_bitmap = displayio.Bitmap(WIDTH, HEIGHT, 1)
color_palette = displayio.Palette(1)
color_palette[0] = 0xFFFFFF  # White

bg_sprite = displayio.TileGrid(color_bitmap, pixel_shader=color_palette, x=0, y=0)
splash.append(bg_sprite)

#Draw a smaller inner rectangle in black
inner_bitmap = displayio.Bitmap(WIDTH - BORDER * 2, HEIGHT - BORDER * 2, 1)
inner_palette = displayio.Palette(1)
inner_palette[0] = 0x000000  # Black
inner_sprite = displayio.TileGrid(inner_bitmap, pixel_shader=inner_palette, x=BORDER, y=BORDER)
splash.append(inner_sprite)

#Draw some white squares
sm_bitmap = displayio.Bitmap(8, 8, 1)
sm_square = displayio.TileGrid(sm_bitmap, pixel_shader=color_palette, x=58, y=17)
splash.append(sm_square)

med_bitmap = displayio.Bitmap(16, 16, 1)
med_square = displayio.TileGrid(med_bitmap, pixel_shader=color_palette, x=71, y=15)
splash.append(med_square)

lrg_bitmap = displayio.Bitmap(32, 32, 1)
lrg_square = displayio.TileGrid(lrg_bitmap, pixel_shader=color_palette, x=91, y=28)
splash.append(lrg_square)

#Draw some label text
text1 = "Hello there"  # overly long to see where it clips
text_area = label.Label(terminalio.FONT, text=text1, color=0xFFFFFF, x=8, y=8)
splash.append(text_area)
text2 = "WWWWHAAAAAAAAA"
text_area2 = label.Label(terminalio.FONT, text=text2, scale=2, color=0xFFFFFF, x=9, y=44)
splash.append(text_area2)

while True:
    pass







# Passive Reconnaisance Section:

### Device Name: Adafruit Fruit Jam - Mini RP2350 Computer

### Manufacturer: Adafruit Industries

### Device Type: Single-Board Computer (SBC) / Microcontroller Development Board

### Video: DVI Video port. This provides digital video output (up to 720p) driven by the RP2350's HSTX interface.

# Audio:

### Stereo Headphone: 3.5mm analog audio jack.

### 4-8$\Omega$ Speaker: 2-pin JST connector for a mono speaker.

### Audio DAC: The board features a TLV320DAC3100 I2S audio digital-to-analog converter to drive the headphone and speaker outputs.

# USB:

### 2x USB-A ports: Provides USB host capabilities for connecting peripherals like a keyboard, mouse, or game controller.

### 1x USB-C port (unlabeled, near MicroSD): Used for power, programming (UF2 bootloader), and USB client functionality.

### On-Board Peripherals & User Interface
### User Buttons: Three tactile switches labeled Button #1, Button #2, and Button #3.

### Reset Button: A single tactile switch for resetting the microcontroller.

### Power Switch: A slide switch labeled ON / OFF.

### Visual Indicators:

### NeoPixels: A connector and likely on-board addressable RGB LEDs (confirmed as 5x NeoPixels).

# Sensors:

### IR In: An infrared receiver for remote control.

### Expansion & Debugging
### GPIO Header: A 16-pin header providing access to digital (D-pins), analog (A-pins), and power (3V, 5V, GND) pins.

### SWD Debug: A Serial Wire Debug port for advanced programming and debugging.

### I2C Connector: A 4-pin JST-SH connector (identified as Stemma QT) for connecting I2C sensors and peripherals.

### Programming: The UF2 label indicates support for the drag-and-drop UF2 bootloader.
