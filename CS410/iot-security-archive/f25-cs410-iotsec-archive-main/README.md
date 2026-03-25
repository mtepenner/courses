# IoT Security Labs: LoRa Packet Sniffer 📡🕵️‍♂️

**Author:** Matthew Penner  
**Term:** Fall 2025  

## 📖 Overview
This repository contains a progressive series of IoT security and hardware development labs. It documents the journey from raw component soldering to building a fully functional **LoRa Packet Sniffer**. Built on the Adafruit Feather RP2040 RFM and written in CircuitPython, the final project captures live LoRa network traffic, displays live statistics on an OLED screen, and streams raw, byte-packed PCAP data over a secondary serial port directly into Wireshark for analysis.

## 🛠 Hardware & Tech Stack
* **Microcontroller:** Adafruit Feather RP2040 RFM (with built-in LoRa radio)
* **Co-Processor:** Adafruit AirLift WiFi FeatherWing (ESP32)
* **Display:** Adafruit 128x64 Monochrome OLED FeatherWing (SH1107)
* **Language:** CircuitPython (v10.x)
* **Key Libraries:** `usb_cdc`, `struct`, `adafruit_rfm9x`, `adafruit_displayio_sh1107`, `adafruit_esp32spi`
* **Analysis Tool:** Wireshark

## 🚀 Lab Progression

### Lab 0: Hardware Assembly & Soldering
The foundation of the project. Involved soldering the Adafruit Feather RP2040, the AirLift WiFi Co-Processor, and the OLED display onto a Feather Tripler board to create a unified hardware testing rig. 

### Lab 1: Radio Communication Testing
Initial testing of the RP2040's radio capabilities. This lab focused on establishing basic sender/receiver roles between two devices, troubleshooting USB data transfer issues, and verifying packet reception via the CircuitPython REPL.

### Lab 2: Network Connectivity & Display Initialization
Focused on writing drivers and testing peripheral capabilities:
* **Wi-Fi Reconnaissance:** Interfaced with the ESP32 AirLift shield using SPI to scan local networks, connect to access points, and fetch text/JSON payloads from Adafruit test servers.
* **OLED Interfacing:** Used `displayio` and the I2C bus to initialize the SH1107 screen, drawing custom bitmaps, shapes, and text labels to prepare for the live UI.

### Lab 3: Building the LoRa Packet Sniffer
Upgraded the basic radio into a live packet sniffing tool with direct Wireshark integration:
* **Dual Serial Ports:** Configured `boot.py` to enable a secondary USB CDC data port, successfully separating REPL debugging output from the raw data stream.
* **Live UI:** Programmed the OLED to display a live packet count and RSSI (signal strength) metrics.
* **PCAP Generation:** Utilized the `struct` library to dynamically pack Python data into C-style binary byte structures, constructing standard PCAP global headers and per-packet record headers.

## 💻 Usage & Wireshark Workflow

To use the packet sniffer and stream data to Wireshark on a Linux machine:

1. Flash the board with the project files (ensure `boot.py` enables `usb_cdc.data`).
2. Connect your code editor (e.g., Thonny) to the first serial port (console) for standard output.
3. Pipe the secondary data port directly into Wireshark using the following terminal command:
   ```bash
   cat /dev/ttyACM1 | wireshark -k -i -
