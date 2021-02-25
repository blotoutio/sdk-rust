# System Events

## Required events
Our SDK only needs to send one event by default. That is triggered when SDK loads.

#### SDK start
`sdk_start` event is triggered as soon as initialization function is called via [`initializeAnalyticsEngine`](/api.md#init) api. This allows us to record a user.