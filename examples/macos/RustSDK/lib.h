#ifndef BLOTOUT_RUST_FFI_H
#define BLOTOUT_RUST_FFI_H


#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

void bo_init(const char *token, const char *end_point);

void bo_capture(const char *event_name, const char *json_string);

void bo_log_pii_event(const char *event_name, const char *json_string);

void bo_log_phi_event(const char *event_name, const char *json_string);

void bo_session_end(void);

void bo_enable_logd(bool log_enabled);

#endif /* BLOTOUT_RUST_FFI_H */
