# BuilderUsage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cpu_milli_max** | Option<**i32**> | Highest CPU use seen, in millicores. | [optional]
**cpu_milli_avg** | Option<**i32**> | Average CPU use over the samples, in millicores. | [optional]
**memory_mib_max** | Option<**i32**> | Highest memory use seen, in MiB. | [optional]
**samples** | Option<**i32**> | Number of samples the values above come from. Usage is sampled periodically, so a short build has few samples and its peaks can be missed.  | [optional]
**oom_killed** | Option<**bool**> | Present, and true, only when the builder was killed for running out of memory. Absent otherwise: never test it against false. Raising the build memory setting fixes it.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


