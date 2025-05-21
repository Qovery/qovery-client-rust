# CronJobResponseAllOfScheduleCronjob

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**arguments** | Option<**Vec<String>**> |  | [optional]
**entrypoint** | Option<**String**> | optional entrypoint when launching container | [optional]
**timezone** | **String** | tz identifier at which the schedule at will be executed | 
**scheduled_at** | **String** | Can only be set if the event is CRON.   Represent the cron format for the job schedule without seconds.   For example: `* * * * *` represent the cron to launch the job every minute.   See https://crontab.guru/ to WISIWIG interface.   Timezone is UT  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


