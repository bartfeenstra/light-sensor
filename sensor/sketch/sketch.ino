#include <pb_encode.h>
#include <pb_common.h>
#include <pb.h>

byte INPUT_MOVEMENT = 3;
byte INPUT_LUMINOSITY = 0;
byte HEADER = 0x7E;
byte packet[4] = {HEADER};

void setup() {
  pinMode(INPUT_MOVEMENT, INPUT);
  pinMode(LED_BUILTIN, OUTPUT);
  Serial.begin(9600);
  // Ensure the USB serial port is connected.
  while (!Serial) {
    ;
  }
}

void loop() {
  byte movement = digitalRead(INPUT_MOVEMENT);
  int luminosity = analogRead(INPUT_LUMINOSITY);
  packet[1] = movement;
  packet[2] = luminosity & 255;
  packet[3] = luminosity >> 8 & 255;
  Serial.write(packet, sizeof(packet));
  delay(100);
}
