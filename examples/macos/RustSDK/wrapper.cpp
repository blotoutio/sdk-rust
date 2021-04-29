#include "wrapper.hpp"
#include <iostream>
using namespace std;

extern "C" {
#include "lib.h"
}

namespace blotout {

//default constructor
BlotoutAnalytics::BlotoutAnalytics() {

}

BlotoutAnalytics::~BlotoutAnalytics() {

}

 void BlotoutAnalytics::initSDK(const char* token, const char* end_point) {
      bo_init(token, end_point);
  }

  void BlotoutAnalytics::logEvent(const char* event_name,const char* json_string) {
      bo_capture(event_name,json_string);
  }


  void BlotoutAnalytics::logPiiEvent(const char*event_name,const char* json_string) {
      bo_log_pii_event(event_name,json_string);
  }

  void BlotoutAnalytics::logPhiEvent(const char*event_name, const char* json_string) {
      bo_log_phi_event(event_name,json_string);

  }

 void  BlotoutAnalytics::sessionEnd() {
     bo_session_end();
  }

void  BlotoutAnalytics::logEnabled(bool log_enabled) {
     bo_enable_log(log_enabled);
  }


}
