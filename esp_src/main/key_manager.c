#include <stdlib.h>
#include "esp_log.h"
#include "tinyusb.h"
#include "freertos/task.h"
#include "class/hid/hid_device.h"

#include "key_manager.h"

static const char *TAG = "key_manager";

uint8_t pressed_keys[6] = {0, 0, 0, 0, 0, 0};
// Index of the first unpressed location in the buffer
uint8_t key_index;
uint8_t has_changed;

void queue_key(uint8_t keycode) {
    ESP_LOGI(TAG, "enqueueing key");

    // Ensure the key isn't already queued
    for (int i = 0; i < key_index; ++i) {
        if (pressed_keys[i] == keycode) {
            return;
        }
    }

    if (key_index < 6) {
        pressed_keys[key_index] = keycode;
        ++key_index;
    }
    has_changed = 1;
}

void dequeue_key(uint8_t keycode) {
    ESP_LOGI(TAG, "dequeueing key");
    for (int i = 0; i < key_index; ++i) {
        if (pressed_keys[i] == keycode) {
            pressed_keys[i] = HID_KEY_NONE;
            // Shift trailing keys back to overwrite position i
            for (int j = i + 1; j < key_index; ++j) {
                pressed_keys[j - 1] = pressed_keys[j];
                pressed_keys[j] = HID_KEY_NONE;
            }

            --key_index;
            has_changed = 1;
            return;
        }
    }
}

void send_report(void) {
    ESP_LOGI(TAG, "Sending Keyboard report {%i, %i, %i, %i, %i, %i}", pressed_keys[0], pressed_keys[1], pressed_keys[2],
             pressed_keys[3], pressed_keys[4], pressed_keys[5]);

    tud_hid_keyboard_report(HID_ITF_PROTOCOL_KEYBOARD, 0, pressed_keys);
    // vTaskDelay(pdMS_TO_TICKS(50));
    // tud_hid_keyboard_report(HID_ITF_PROTOCOL_KEYBOARD, 0, NULL);
    has_changed = 0;
}
