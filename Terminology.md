# Terminology notes

Below are what the author understands of certain concepts of the EZSP API. They may be wrong - please correct.

<!--tbd. Based on: <links>-->

**ASH**

*"handles retries and ensures non-cryptographic packet integrity."* [AN1125]

*"The ASH framework handles the encapsulation of EZSP frames into a more robust UART protocol and occasionally does some of its own serial transactions for QoS purposes."*


**Pan id**

<font color=red>( ) What is it?</font>



**Manufacturing token**


**EUI64 ID**


**EmberZNet PRO**

The stack that runs within the *Sonoff Zigbee 3.0 Plus V2* dongle. It's a full Zigbee stack that you can control via the serial (EZSP) interface.

It's all on the side of the NCP, so basically "you don't need to know". :)

**NCP**

Network Co-Processor. What the Sonoff dongle is to the PC it's connected with.

