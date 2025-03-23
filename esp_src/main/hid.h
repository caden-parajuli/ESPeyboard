#include <stdlib.h>
#include "freertos/FreeRTOS.h"
#include "class/hid/hid_device.h"

extern const uint8_t hid_report_descriptor[];
extern const char *hid_string_descriptor[5];
extern const uint8_t hid_configuration_descriptor[];

uint8_t const *tud_hid_descriptor_report_cb(uint8_t instance);

uint16_t tud_hid_get_report_cb(uint8_t instance, uint8_t report_id, hid_report_type_t report_type, uint8_t *buffer,
                               uint16_t reqlen);

void tud_hid_set_report_cb(uint8_t instance, uint8_t report_id, hid_report_type_t report_type, uint8_t const *buffer,
                           uint16_t bufsize);
