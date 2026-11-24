# SkillPresetRejectionFinding

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**source** | [**models::SkillPresetFindingSource**](SkillPresetFindingSource.md) |  | 
**rule_id** | **String** | For `source` RULES, the rule that matched, for example `override-instructions`, `conceal-from-user`, `skip-confirmation`, `read-credentials`, `dump-secrets`, `exfiltration-endpoint`, `upload-command`, `remote-exec`, `obfuscated-exec`, `external-url`, `invisible-characters`, `html-comment`, `encoded-blob` or `credential`. For `source` MODEL, the category: `prompt-injection`, `concealment`, `credential-access`, `data-exfiltration`, `unsafe-execution`, `approval-bypass`, `social-engineering` or `other`. New rules and categories can be added. | 
**location** | **String** | The field that holds the finding: `description`, `team context`, `description of skill '<name>'` or `content of skill '<name>'`. | 
**line** | **i32** | 1-based line within that field. | 
**reason** | **String** | Why the content was refused. Never contains the offending text. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


