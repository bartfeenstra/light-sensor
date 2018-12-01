#include "telemetry.pb.h"

#include <pb_encode.h>
#include <pb_common.h>
#include <pb.h>

byte INPUT_MOVEMENT = 3;
byte INPUT_LUMINOSITY = 0;

void setup() {
  pinMode(INPUT_MOVEMENT, INPUT);
  pinMode(LED_BUILTIN, OUTPUT);
  Serial.begin(9600);
  // Do nothing (but wait) until the USB serial port is connected.
  while (!Serial) {
    ;
  }
}

void loop() {
  Telemetry telemetry = Telemetry_init_zero;
  telemetry.movement = digitalRead(INPUT_MOVEMENT);
  telemetry.luminosity = analogRead(INPUT_LUMINOSITY);
  uint8_t buffer[Telemetry_size];
  pb_ostream_t stream = pb_ostream_from_buffer(buffer, sizeof(buffer));
  if (pb_encode(&stream, Telemetry_fields, &telemetry)) {
    Serial.write(buffer, stream.bytes_written);
  }
  delay(100);
}
