idf_component_register(
    SRCS "main.c" "key_manager.c" "hid.c"
    INCLUDE_DIRS "."
    PRIV_REQUIRES esp_driver_gpio
    )
