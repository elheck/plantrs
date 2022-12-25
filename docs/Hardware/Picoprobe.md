[[Setup Moc]]

-  See the pico probe chapter in the [getting started guide ]()https://datasheets.raspberrypi.com/pico/getting-started-with-pico.pdf 
- All you have to do is to flash the picoprobe software onto a [[Raspberry Pi Pico]] and connect it according to [this tutorial](https://www.digikey.de/de/maker/projects/raspberry-pi-pico-and-rp2040-cc-part-2-debugging-with-vs-code/470abc7efb07432b82c95f6f67f184c0)
	- To do that, first download this [UF2](https://datasheets.raspberrypi.org/soft/picoprobe.uf2) binary onto your development machine
	- Then, holding down the BOOTSEL (short for boot select) button on the [[Raspberry Pi Pico]] that will become the Picoprobe, plug in the USB cable into your development machine
	- A window of the flash storage of the [[Raspberry Pi Pico]] should pop up no matter what operating system you’re running on your development machine
	- Drag and drop the picoprobe.uf2 onto this window
	- The [[Raspberry Pi Pico]] should now be fully flashed and ready to be used as a Picoprobe

![[bed6afe4-af01-4a6a-b2d0-d768934f6ab5.jpeg]]