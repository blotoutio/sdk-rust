using System;
using System.Runtime.InteropServices;

namespace RustCSharp
{
    class Program
    {
        [DllImport("blotout.dll", EntryPoint = "bo_session_end")] private static extern void bo_session_end();
        [DllImport("blotout.dll", EntryPoint = "bo_init")] private static extern void bo_init(string token, string sdk_end_point);
        [DllImport("blotout.dll", EntryPoint = "bo_capture")] private static extern void bo_capture(string event_name, string json_string);
        [DllImport("blotout.dll", EntryPoint = "bo_log_pii_event")] private static extern void bo_log_pii_event(string event_name, string json_string);
        [DllImport("blotout.dll", EntryPoint = "bo_log_phi_event")] private static extern void bo_log_phi_event(string event_name, string json_string);
        [DllImport("blotout.dll", EntryPoint = "bo_enable_log")] private static extern void bo_enable_log(bool log_enabled);

        static void Main(string[] args)
        {
            Console.WriteLine("Hello World!");
            TestSDKInit();
            TestPublishEvents();
            TestPublishPHIEvents();
            TestPublishPIIEvents();
        }

        private static void TestSDKInit()
        {

            string token = "BEZAVGGW4GZZZ3N";
            string endPoint = "http://stage.blotout.io";
            string bundleID = "com.blotout.rustsaleDemoApp";
            bo_init(token, endPoint, bundleID);
        }

        private static void TestPublishEvents()
        {
            string eventName = "Rust Event from sdk testing";

          string jsonString = "{\"some property\": \"some value\", \"some other property\": \"some other value\"}";

            bo_capture(eventName,jsonString);
    }

        private static void TestPublishPIIEvents()
        {
            string eventName = "Rust Event from sdk testing";

            string jsonString = "{\"email id\": \"ankuradhikari08@gmail.com\", \"gender\": \"male\"}";

            bo_log_pii_event(eventName, jsonString);
        }

        private static void TestPublishPHIEvents()
        {
            string eventName = "Rust Event from sdk testing";

            string jsonString = "{\"email id\": \"ankur@blotout.io\", \"gender\": \"male\"}";

            bo_log_phi_event(eventName, jsonString);
        }
    }
}
