# DeploymentHistoryJobResponseAllOfSchedule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**event** | Option<[**models::JobScheduleEvent**](JobScheduleEvent.md)> |  | [optional]
**schedule_at** | Option<**String**> | Can only be set if the event is CRON. Represent the cron format for the job schedule without seconds. For example: `* * * * *` represent the cron to launch the job every minute. See https://crontab.guru/ to WISIWIG interface. Timezone is UTC  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


