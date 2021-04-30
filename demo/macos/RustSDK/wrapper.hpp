#ifndef BLOTOUT_RUST_FFI_SRC_WRAPPER_HPP_
#define BLOTOUT_RUST_FFI_SRC_WRAPPER_HPP_

extern "C" {
    #include "lib.h"
}

#if defined(BLOTOUT_SHARED_LIBRARY)
#if defined(WIN32)
#if defined(BLOTOUT_IMPLEMENTATION)
#define BLOTOUT_EXPORT __declspec(dllexport)
#else
#define BLOTOUT_EXPORT __declspec(dllimport)
#endif  // defined(BLOTOUT_IMPLEMENTATION)
#else  // defined(WIN32)
#if defined(BLOTOUT_IMPLEMENTATION)
#define BLOTOUT_EXPORT __attribute__((visibility("default")))
#else
#define BLOTOUT_EXPORT
#endif  // defined(BLOTOUT_IMPLEMENTATION)
#endif
#else  // defined(BLOTOUT_SHARED_LIBRARY)
#define BLOTOUT_EXPORT
#endif

namespace blotout {

    class BLOTOUT_EXPORT BlotoutAnalytics {

    public:
        BlotoutAnalytics();

        void initSDK(const char *token,
                        const char *end_point);
        void logEvent(const char *event_name,
                            const char *json_string);
        void logPiiEvent(const char *event_name,
                                const char *json_string);
        void logPhiEvent(const char *event_name,
                                const char *json_string);
        void sessionEnd(void);

        void logEnabled(bool log_enabled);

        ~BlotoutAnalytics();
    }; // namespace blotout
}
#endif
