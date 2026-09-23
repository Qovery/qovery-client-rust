# BuildSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**timeout_max_sec** | Option<**i32**> | Maximum build timeout in seconds | [optional][default to 1800]
**cpu_max_in_milli** | Option<**i32**> | Maximum CPU resources for the build (in millicores) | [optional][default to 4000]
**ram_max_in_gib** | Option<**i32**> | Maximum RAM resources for the build (in GiB) | [optional][default to 8]
**ephemeral_storage_in_gib** | Option<**i32**> | Ephemeral storage for the build (in GiB). When null, the platform default is used. | [optional]
**disable_buildkit_cache** | Option<**bool**> | Disable buildkit registry cache during build | [optional][default to false]
**skip_git_submodules** | Option<**bool**> | Skip git submodules update when cloning the repository | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


