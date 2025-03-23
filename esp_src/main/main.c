/*
 * SPDX-FileCopyrightText: 2022-2024 Espressif Systems (Shanghai) CO LTD
 *
 * SPDX-License-Identifier: Unlicense OR CC0-1.0
 */

#include <stdlib.h>
#include "esp_log.h"
#include "freertos/FreeRTOS.h"
#include "freertos/task.h"
#include "tinyusb.h"
#include "class/hid/hid_device.h"
#include "driver/gpio.h"

#include "key_manager.h"
#include "hid.h"

static const char *TAG = "ESPeypoard_main";

#define APP_BUTTON (GPIO_NUM_0) // Use BOOT signal by default

/************* TinyUSB descriptors ****************/


/********* Application ***************/

#define DISTANCE_MAX 125
#define DELTA_SCALAR 5

// static void app_send_hid_demo(void) {
//     // Keyboard output: Send key 'l/L' pressed and released
//     ESP_LOGI(TAG, "Sending Keyboard report");
//
//     // A Key down
//     queue_key(HID_KEY_A);
//     send_keys();
//
//     vTaskDelay(pdMS_TO_TICKS(100));
//
//     // A Key up
//     dequeue_key(HID_KEY_A);
//     send_keys();
// }

void app_main(void) {
    // Initialize button that will trigger HID reports
    const gpio_config_t boot_button_config = {
        .pin_bit_mask = BIT64(APP_BUTTON),
        .mode = GPIO_MODE_INPUT,
        .intr_type = GPIO_INTR_DISABLE,
        .pull_up_en = true,
        .pull_down_en = false,
    };
    ESP_ERROR_CHECK(gpio_config(&boot_button_config));

    ESP_LOGI(TAG, "USB initialization");
    const tinyusb_config_t tusb_cfg = {
        .device_descriptor = NULL,
        .string_descriptor = hid_string_descriptor,
        .string_descriptor_count = sizeof(hid_string_descriptor) / sizeof(hid_string_descriptor[0]),
        .external_phy = false,
#if (TUD_OPT_HIGH_SPEED)
        .fs_configuration_descriptor =
            hid_configuration_descriptor, // HID configuration descriptor for full-speed and high-speed are the same
        .hs_configuration_descriptor = hid_configuration_descriptor,
        .qualifier_descriptor = NULL,
#else
        .configuration_descriptor = hid_configuration_descriptor,
#endif // TUD_OPT_HIGH_SPEED
    };

    ESP_ERROR_CHECK(tinyusb_driver_install(&tusb_cfg));
    ESP_LOGI(TAG, "USB initialization DONE");

    // Main loop
    bool button_pressed = false;
    while (1) {
        if (tud_mounted()) {
            button_pressed = !gpio_get_level(APP_BUTTON);
            if (button_pressed) {
                // app_send_hid_demo();
                queue_key(HID_KEY_B);
            } else {
                dequeue_key(HID_KEY_B);
            }

            send_report();
            // report_if_necessary();
        }
        vTaskDelay(pdMS_TO_TICKS(100));
    }
}
