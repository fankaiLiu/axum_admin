# \DefaultApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**account_check**](DefaultApi.md#account_check) | **Post** /user/account_check | 检查列表账户注册状态，并且返回结果
[**add_black**](DefaultApi.md#add_black) | **Post** /friend/add_black | 添加黑名单
[**add_friend**](DefaultApi.md#add_friend) | **Post** /friend/add_friend | 添加好友
[**add_friend_response**](DefaultApi.md#add_friend_response) | **Post** /friend/add_friend_response | 同意/拒绝好友请求
[**application_group_response**](DefaultApi.md#application_group_response) | **Post** /group/group_application_response | 处理加群消息
[**cancel_mute_group**](DefaultApi.md#cancel_mute_group) | **Post** /group/cancel_mute_group | 取消禁言群组
[**cancel_mute_group_member**](DefaultApi.md#cancel_mute_group_member) | **Post** /group/cancel_mute_group_member | 取消禁言群成员
[**clear_msg**](DefaultApi.md#clear_msg) | **Post** /msg/clear_msg | 清空用户消息
[**comment_one_work_moment**](DefaultApi.md#comment_one_work_moment) | **Post** /office/comment_one_work_moment | 评论一条工作圈
[**create_department**](DefaultApi.md#create_department) | **Post** /organization/create_department | 创建部门
[**create_department_member**](DefaultApi.md#create_department_member) | **Post** /organization/create_department_member | 创建部门用户
[**create_group**](DefaultApi.md#create_group) | **Post** /group/create_group | 创建群组
[**create_one_work_moment**](DefaultApi.md#create_one_work_moment) | **Post** /office/create_one_work_moment | 创建一条工作圈
[**create_organization_user**](DefaultApi.md#create_organization_user) | **Post** /organization/create_organization_user | 组织架构导入用户
[**create_tag**](DefaultApi.md#create_tag) | **Post** /office/create_tag | 创建标签
[**del_msg**](DefaultApi.md#del_msg) | **Post** /msg/del_msg | 根据seq列表删除消息
[**delete_comment**](DefaultApi.md#delete_comment) | **Post** /office/delete_comment | 删除一条评论
[**delete_department**](DefaultApi.md#delete_department) | **Post** /organization/delete_department | 删除部门
[**delete_friend**](DefaultApi.md#delete_friend) | **Post** /friend/delete_friend | 删除好友
[**delete_one_work_moment**](DefaultApi.md#delete_one_work_moment) | **Post** /office/delete_one_work_moment | 删除一条工作圈
[**delete_organization_user**](DefaultApi.md#delete_organization_user) | **Post** /organization/delete_organization_user | 删除组织架构中某个用户
[**delete_tag**](DefaultApi.md#delete_tag) | **Post** /office/delete_tag | 删除标签
[**delete_user_in_department**](DefaultApi.md#delete_user_in_department) | **Post** /organization/delete_user_in_department | 删除部门中某个用户
[**dismiss_group**](DefaultApi.md#dismiss_group) | **Post** /group/dismiss_group | 解散群组
[**force_logout**](DefaultApi.md#force_logout) | **Post** /auth/force_logout | 强制登出
[**get_all_conversations**](DefaultApi.md#get_all_conversations) | **Post** /msg/get_all_conversations | 获取用户所有会话
[**get_all_users_uid**](DefaultApi.md#get_all_users_uid) | **Post** /user/get_all_users_uid | 获取所有用户uid列表
[**get_blacklist**](DefaultApi.md#get_blacklist) | **Post** /friend/get_black_list | 获取黑名单列表
[**get_conversation**](DefaultApi.md#get_conversation) | **Post** /msg/get_conversation | 根据会话ID获取会话
[**get_conversations**](DefaultApi.md#get_conversations) | **Post** /msg/get_conversations | 根据会话ID列表获取会话
[**get_department_member**](DefaultApi.md#get_department_member) | **Post** /organization/get_department_member | 获取部门中所有成员
[**get_friend_apply_list**](DefaultApi.md#get_friend_apply_list) | **Post** /friend/get_friend_apply_list | 获取好友申请列表
[**get_friend_list**](DefaultApi.md#get_friend_list) | **Post** /friend/get_friend_list | 获取用户的好友列表
[**get_group_all_member_list**](DefaultApi.md#get_group_all_member_list) | **Post** /group/get_group_all_member_list | 获取全部群成员列表
[**get_group_members_info**](DefaultApi.md#get_group_members_info) | **Post** /group/get_group_members_info | 获取群成员信息
[**get_groups_info**](DefaultApi.md#get_groups_info) | **Post** /group/get_groups_info | 通过群ID列表获取群信息
[**get_joined_group_list**](DefaultApi.md#get_joined_group_list) | **Post** /group/get_joined_group_list | 获取用户加入群列表
[**get_recv_group_application_list**](DefaultApi.md#get_recv_group_application_list) | **Post** /group/get_recv_group_applicationList | 获取用户收到的加群信息列表
[**get_self_friend_apply_list**](DefaultApi.md#get_self_friend_apply_list) | **Post** /friend/get_self_friend_apply_list | 获取自己的好友申请列表
[**get_self_user_info**](DefaultApi.md#get_self_user_info) | **Post** /user/get_self_user_info | 获取自己的信息
[**get_sub_department**](DefaultApi.md#get_sub_department) | **Post** /organization/get_sub_department | 获取子部门列表
[**get_tag_send_logs**](DefaultApi.md#get_tag_send_logs) | **Post** /office/get_send_tag_log | 获取发送历史记录
[**get_user_friend_work_moments**](DefaultApi.md#get_user_friend_work_moments) | **Post** /office/get_user_friend_work_moments | 查询自己大工作圈页面
[**get_user_in_department**](DefaultApi.md#get_user_in_department) | **Post** /organization/get_user_in_department | 获取部门中的所有用户
[**get_user_req_group_application_list**](DefaultApi.md#get_user_req_group_application_list) | **Post** /group/get_user_req_group_applicationList | 获取用户加群申请列表
[**get_user_tag_by_id**](DefaultApi.md#get_user_tag_by_id) | **Post** /office/get_user_tag_by_id | 获取该用户的标签信息
[**get_user_tags**](DefaultApi.md#get_user_tags) | **Post** /office/get_user_tags | 获取用户标签信息
[**get_user_work_moments**](DefaultApi.md#get_user_work_moments) | **Post** /office/get_user_work_moments | 查询用户工作圈
[**get_users_info**](DefaultApi.md#get_users_info) | **Post** /user/get_users_info | 获取用户信息
[**get_users_online_status**](DefaultApi.md#get_users_online_status) | **Post** /user/get_users_online_status | 获取用户在线状态
[**get_work_moment_by_id**](DefaultApi.md#get_work_moment_by_id) | **Post** /office/get_work_moment_by_id | 通过ID获取工作圈
[**import_friend**](DefaultApi.md#import_friend) | **Post** /friend/import_friend | 批量加好友
[**invite_user_to_group**](DefaultApi.md#invite_user_to_group) | **Post** /group/invite_user_to_group | 将用户拉入群组
[**is_friend**](DefaultApi.md#is_friend) | **Post** /friend/is_friend | 检查用户之间是否为好友
[**join_group**](DefaultApi.md#join_group) | **Post** /group/join_group | 加入群聊
[**kick_group_member**](DefaultApi.md#kick_group_member) | **Post** /group/kick_group | 把用户踢出群组
[**like_one_work_moment**](DefaultApi.md#like_one_work_moment) | **Post** /office/like_one_work_moment | 点赞一条工作圈
[**management_batch_send_msg**](DefaultApi.md#management_batch_send_msg) | **Post** /msg/batch_send_msg | 管理员批量发送群聊单聊消息
[**management_send_msg**](DefaultApi.md#management_send_msg) | **Post** /msg/manage_send_msg | 管理员发送/撤回消息
[**minio_upload_file**](DefaultApi.md#minio_upload_file) | **Post** /third/minio_upload | minio上传文件(web api)
[**mute_group**](DefaultApi.md#mute_group) | **Post** /group/mute_group | 禁言群组
[**mute_group_member**](DefaultApi.md#mute_group_member) | **Post** /group/mute_group_member | 禁言群成员
[**parse_token**](DefaultApi.md#parse_token) | **Post** /auth/parse_token | 解析当前用户token
[**quit_group**](DefaultApi.md#quit_group) | **Post** /group/quit_group | 当前用户退出群聊
[**remove_black**](DefaultApi.md#remove_black) | **Post** /friend/remove_black | 把用户移除黑名单
[**send_msg2_tag**](DefaultApi.md#send_msg2_tag) | **Post** /office/send_msg_to_tag | 发送标签消息
[**set_friend_remark**](DefaultApi.md#set_friend_remark) | **Post** /friend/set_friend_remark | 设置好友备注
[**set_global_recv_message_opt**](DefaultApi.md#set_global_recv_message_opt) | **Post** /user/set_global_msg_recv_opt | 设置全局免打扰
[**set_group_info**](DefaultApi.md#set_group_info) | **Post** /group/set_group_info | 设置群信息
[**set_group_member_info**](DefaultApi.md#set_group_member_info) | **Post** /group/set_group_member_info | 修改群成员信息
[**set_tag**](DefaultApi.md#set_tag) | **Post** /office/set_tag | 修改标签
[**transfer_group_owner**](DefaultApi.md#transfer_group_owner) | **Post** /group/transfer_group | 转让群主
[**update_department**](DefaultApi.md#update_department) | **Post** /organization/update_department | 更新部门信息
[**update_organization_user**](DefaultApi.md#update_organization_user) | **Post** /organization/update_organization_user | 更新组织架构中的用户
[**update_user_in_department**](DefaultApi.md#update_user_in_department) | **Post** /organization/update_user_in_department | 更新部门中某个用户
[**update_user_info**](DefaultApi.md#update_user_info) | **Post** /user/update_user_info | 修改用户信息
[**user_register**](DefaultApi.md#user_register) | **Post** /auth/user_register | 用户注册
[**user_token**](DefaultApi.md#user_token) | **Post** /auth/user_token | 用户登录



## account_check

> crate::models::BaseInfoAccountCheckResp account_check(token, base_info_account_check_req)
检查列表账户注册状态，并且返回结果

传入UserIDList检查列表账户注册状态，并且返回结果

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_account_check_req** | Option<[**BaseInfoAccountCheckReq**](BaseInfoAccountCheckReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoAccountCheckResp**](BaseInfoAccountCheckResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_black

> crate::models::BaseInfoAddBlacklistResp add_black(token, base_info_add_blacklist_req)
添加黑名单

添加黑名单

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_add_blacklist_req** | Option<[**BaseInfoAddBlacklistReq**](BaseInfoAddBlacklistReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoAddBlacklistResp**](BaseInfoAddBlacklistResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_friend

> crate::models::BaseInfoAddFriendResp add_friend(token, base_info_add_friend_req)
添加好友

添加好友

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_add_friend_req** | Option<[**BaseInfoAddFriendReq**](BaseInfoAddFriendReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoAddFriendResp**](BaseInfoAddFriendResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_friend_response

> crate::models::BaseInfoAddFriendResponseResp add_friend_response(token, base_info_add_friend_response_req)
同意/拒绝好友请求

同意/拒绝好友请求

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_add_friend_response_req** | Option<[**BaseInfoAddFriendResponseReq**](BaseInfoAddFriendResponseReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoAddFriendResponseResp**](BaseInfoAddFriendResponseResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## application_group_response

> crate::models::BaseInfoApplicationGroupResponseResp application_group_response(token, base_info_application_group_response_req)
处理加群消息

处理加群消息

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_application_group_response_req** | Option<[**BaseInfoApplicationGroupResponseReq**](BaseInfoApplicationGroupResponseReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoApplicationGroupResponseResp**](BaseInfoApplicationGroupResponseResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cancel_mute_group

> crate::models::BaseInfoCancelMuteGroupResp cancel_mute_group(token, base_info_cancel_mute_group_req)
取消禁言群组

取消禁言群组

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_cancel_mute_group_req** | Option<[**BaseInfoCancelMuteGroupReq**](BaseInfoCancelMuteGroupReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoCancelMuteGroupResp**](BaseInfoCancelMuteGroupResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cancel_mute_group_member

> crate::models::BaseInfoCancelMuteGroupMemberResp cancel_mute_group_member(token, base_info_cancel_mute_group_member_req)
取消禁言群成员

取消禁言群成员

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_cancel_mute_group_member_req** | Option<[**BaseInfoCancelMuteGroupMemberReq**](BaseInfoCancelMuteGroupMemberReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoCancelMuteGroupMemberResp**](BaseInfoCancelMuteGroupMemberResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clear_msg

> crate::models::BaseInfoCleanUpMsgResp clear_msg(token, base_info_clean_up_msg_req)
清空用户消息

清空用户消息

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_clean_up_msg_req** | Option<[**BaseInfoCleanUpMsgReq**](BaseInfoCleanUpMsgReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoCleanUpMsgResp**](BaseInfoCleanUpMsgResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## comment_one_work_moment

> crate::models::BaseInfoCommentOneWorkMomentResp comment_one_work_moment(token, base_info_comment_one_work_moment_req)
评论一条工作圈

评论一条工作圈

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_comment_one_work_moment_req** | Option<[**BaseInfoCommentOneWorkMomentReq**](BaseInfoCommentOneWorkMomentReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoCommentOneWorkMomentResp**](BaseInfoCommentOneWorkMomentResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_department

> crate::models::BaseInfoCreateDepartmentResp create_department(token, base_info_create_department_req)
创建部门

创建部门

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_create_department_req** | Option<[**BaseInfoCreateDepartmentReq**](BaseInfoCreateDepartmentReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoCreateDepartmentResp**](BaseInfoCreateDepartmentResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_department_member

> crate::models::BaseInfoCreateDepartmentMemberResp create_department_member(token, base_info_create_department_member_req)
创建部门用户

创建部门用户

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_create_department_member_req** | Option<[**BaseInfoCreateDepartmentMemberReq**](BaseInfoCreateDepartmentMemberReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoCreateDepartmentMemberResp**](BaseInfoCreateDepartmentMemberResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_group

> crate::models::BaseInfoCreateGroupResp create_group(token, base_info_create_group_req)
创建群组

创建群组

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_create_group_req** | Option<[**BaseInfoCreateGroupReq**](BaseInfoCreateGroupReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoCreateGroupResp**](BaseInfoCreateGroupResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_one_work_moment

> crate::models::BaseInfoCreateOneWorkMomentResp create_one_work_moment(token, base_info_create_one_work_moment_req)
创建一条工作圈

用户创建一条工作圈

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_create_one_work_moment_req** | Option<[**BaseInfoCreateOneWorkMomentReq**](BaseInfoCreateOneWorkMomentReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoCreateOneWorkMomentResp**](BaseInfoCreateOneWorkMomentResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_organization_user

> crate::models::BaseInfoCreateOrganizationUserResp create_organization_user(token, base_info_create_organization_user_req)
组织架构导入用户

组织架构导入用户

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_create_organization_user_req** | Option<[**BaseInfoCreateOrganizationUserReq**](BaseInfoCreateOrganizationUserReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoCreateOrganizationUserResp**](BaseInfoCreateOrganizationUserResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_tag

> crate::models::BaseInfoCreateTagResp create_tag(token, base_info_create_tag_req)
创建标签

创建标签

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_create_tag_req** | Option<[**BaseInfoCreateTagReq**](BaseInfoCreateTagReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoCreateTagResp**](BaseInfoCreateTagResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## del_msg

> crate::models::BaseInfoDelMsgResp del_msg(token, base_info_del_msg_req)
根据seq列表删除消息

根据seq列表删除消息

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_del_msg_req** | Option<[**BaseInfoDelMsgReq**](BaseInfoDelMsgReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoDelMsgResp**](BaseInfoDelMsgResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_comment

> crate::models::BaseInfoDeleteCommentResp delete_comment(token, base_info_delete_comment_req)
删除一条评论

删除一条评论

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_delete_comment_req** | Option<[**BaseInfoDeleteCommentReq**](BaseInfoDeleteCommentReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoDeleteCommentResp**](BaseInfoDeleteCommentResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_department

> crate::models::BaseInfoDeleteDepartmentResp delete_department(token, base_info_delete_department_req)
删除部门

删除部门

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_delete_department_req** | Option<[**BaseInfoDeleteDepartmentReq**](BaseInfoDeleteDepartmentReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoDeleteDepartmentResp**](BaseInfoDeleteDepartmentResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_friend

> crate::models::BaseInfoDeleteFriendResp delete_friend(token, base_info_delete_friend_req)
删除好友

删除好友

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_delete_friend_req** | Option<[**BaseInfoDeleteFriendReq**](BaseInfoDeleteFriendReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoDeleteFriendResp**](BaseInfoDeleteFriendResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_one_work_moment

> crate::models::BaseInfoDeleteOneWorkMomentResp delete_one_work_moment(token, base_info_delete_one_work_moment_req)
删除一条工作圈

根据用户工作圈ID删除一条工作圈

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_delete_one_work_moment_req** | Option<[**BaseInfoDeleteOneWorkMomentReq**](BaseInfoDeleteOneWorkMomentReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoDeleteOneWorkMomentResp**](BaseInfoDeleteOneWorkMomentResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_organization_user

> crate::models::BaseInfoDeleteOrganizationUserResp delete_organization_user(token, base_info_delete_organization_user_req)
删除组织架构中某个用户

删除组织架构中某个用户

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_delete_organization_user_req** | Option<[**BaseInfoDeleteOrganizationUserReq**](BaseInfoDeleteOrganizationUserReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoDeleteOrganizationUserResp**](BaseInfoDeleteOrganizationUserResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_tag

> crate::models::BaseInfoDeleteTagResp delete_tag(token, base_info_delete_tag_req)
删除标签

根据标签ID创建标签

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_delete_tag_req** | Option<[**BaseInfoDeleteTagReq**](BaseInfoDeleteTagReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoDeleteTagResp**](BaseInfoDeleteTagResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_in_department

> crate::models::BaseInfoDeleteUserInDepartmentResp delete_user_in_department(token, base_info_delete_user_in_department_req)
删除部门中某个用户

删除部门中某个用户

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_delete_user_in_department_req** | Option<[**BaseInfoDeleteUserInDepartmentReq**](BaseInfoDeleteUserInDepartmentReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoDeleteUserInDepartmentResp**](BaseInfoDeleteUserInDepartmentResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dismiss_group

> crate::models::BaseInfoDismissGroupResp dismiss_group(token, base_info_dismiss_group_req)
解散群组

解散群组

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_dismiss_group_req** | Option<[**BaseInfoDismissGroupReq**](BaseInfoDismissGroupReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoDismissGroupResp**](BaseInfoDismissGroupResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## force_logout

> crate::models::BaseInfoForceLogoutResp force_logout(token, base_info_force_logout_req)
强制登出

对应的平台强制登出

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_force_logout_req** | Option<[**BaseInfoForceLogoutReq**](BaseInfoForceLogoutReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoForceLogoutResp**](BaseInfoForceLogoutResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_conversations

> crate::models::BaseInfoGetAllConversationsResp get_all_conversations(token, base_info_get_all_conversations_req)
获取用户所有会话

获取用户所有会话

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_get_all_conversations_req** | Option<[**BaseInfoGetAllConversationsReq**](BaseInfoGetAllConversationsReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoGetAllConversationsResp**](BaseInfoGetAllConversationsResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_users_uid

> crate::models::BaseInfoGetAllUsersUidResp get_all_users_uid(token, base_info_get_all_users_uid_req)
获取所有用户uid列表

获取所有用户uid列表

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_get_all_users_uid_req** | Option<[**BaseInfoGetAllUsersUidReq**](BaseInfoGetAllUsersUidReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoGetAllUsersUidResp**](BaseInfoGetAllUsersUidResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_blacklist

> crate::models::BaseInfoGetBlackListResp get_blacklist(token, base_info_get_black_list_req)
获取黑名单列表

获取黑名单列表

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_get_black_list_req** | Option<[**BaseInfoGetBlackListReq**](BaseInfoGetBlackListReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoGetBlackListResp**](BaseInfoGetBlackListResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversation

> crate::models::BaseInfoGetConversationResp get_conversation(token, base_info_get_conversation_req)
根据会话ID获取会话

根据会话ID获取会话

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_get_conversation_req** | Option<[**BaseInfoGetConversationReq**](BaseInfoGetConversationReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoGetConversationResp**](BaseInfoGetConversationResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversations

> crate::models::BaseInfoGetConversationsResp get_conversations(token, base_info_get_conversations_req)
根据会话ID列表获取会话

根据会话ID列表获取会话

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_get_conversations_req** | Option<[**BaseInfoGetConversationsReq**](BaseInfoGetConversationsReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoGetConversationsResp**](BaseInfoGetConversationsResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_department_member

> crate::models::BaseInfoGetDepartmentMemberResp get_department_member(token, base_info_get_department_member_req)
获取部门中所有成员

获取部门中所有成员

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_get_department_member_req** | Option<[**BaseInfoGetDepartmentMemberReq**](BaseInfoGetDepartmentMemberReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoGetDepartmentMemberResp**](BaseInfoGetDepartmentMemberResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_friend_apply_list

> crate::models::BaseInfoGetFriendApplyListResp get_friend_apply_list(token, base_info_get_friend_apply_list_req)
获取好友申请列表

删除好友

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_get_friend_apply_list_req** | Option<[**BaseInfoGetFriendApplyListReq**](BaseInfoGetFriendApplyListReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoGetFriendApplyListResp**](BaseInfoGetFriendApplyListResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_friend_list

> crate::models::BaseInfoGetFriendListResp get_friend_list(token, base_info_get_friend_list_req)
获取用户的好友列表

获取用户的好友列表

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_get_friend_list_req** | Option<[**BaseInfoGetFriendListReq**](BaseInfoGetFriendListReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoGetFriendListResp**](BaseInfoGetFriendListResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_all_member_list

> crate::models::BaseInfoGetGroupAllMemberResp get_group_all_member_list(token, base_info_get_group_all_member_req)
获取全部群成员列表

获取全部群成员列表

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_get_group_all_member_req** | Option<[**BaseInfoGetGroupAllMemberReq**](BaseInfoGetGroupAllMemberReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoGetGroupAllMemberResp**](BaseInfoGetGroupAllMemberResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_members_info

> crate::models::BaseInfoGetGroupMembersInfoResp get_group_members_info(token, base_info_get_group_members_info_req)
获取群成员信息

获取群成员信息

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_get_group_members_info_req** | Option<[**BaseInfoGetGroupMembersInfoReq**](BaseInfoGetGroupMembersInfoReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoGetGroupMembersInfoResp**](BaseInfoGetGroupMembersInfoResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_groups_info

> crate::models::BaseInfoGetGroupInfoResp get_groups_info(token, base_info_get_group_info_req)
通过群ID列表获取群信息

通过群ID列表获取群信息

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_get_group_info_req** | Option<[**BaseInfoGetGroupInfoReq**](BaseInfoGetGroupInfoReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoGetGroupInfoResp**](BaseInfoGetGroupInfoResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_joined_group_list

> crate::models::BaseInfoGetJoinedGroupListResp get_joined_group_list(token, base_info_get_joined_group_list_req)
获取用户加入群列表

获取用户加入群列表

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_get_joined_group_list_req** | Option<[**BaseInfoGetJoinedGroupListReq**](BaseInfoGetJoinedGroupListReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoGetJoinedGroupListResp**](BaseInfoGetJoinedGroupListResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_recv_group_application_list

> crate::models::BaseInfoGetGroupApplicationListResp get_recv_group_application_list(token, base_info_get_group_application_list_req)
获取用户收到的加群信息列表

获取用户收到的加群信息列表

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_get_group_application_list_req** | Option<[**BaseInfoGetGroupApplicationListReq**](BaseInfoGetGroupApplicationListReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoGetGroupApplicationListResp**](BaseInfoGetGroupApplicationListResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_self_friend_apply_list

> crate::models::BaseInfoGetSelfApplyListResp get_self_friend_apply_list(token, base_info_get_self_apply_list_req)
获取自己的好友申请列表

获取自己的好友申请列表

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_get_self_apply_list_req** | Option<[**BaseInfoGetSelfApplyListReq**](BaseInfoGetSelfApplyListReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoGetSelfApplyListResp**](BaseInfoGetSelfApplyListResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_self_user_info

> crate::models::BaseInfoGetSelfUserInfoResp get_self_user_info(token, base_info_get_self_user_info_req)
获取自己的信息

传入ID获取自己的信息

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_get_self_user_info_req** | Option<[**BaseInfoGetSelfUserInfoReq**](BaseInfoGetSelfUserInfoReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoGetSelfUserInfoResp**](BaseInfoGetSelfUserInfoResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sub_department

> crate::models::BaseInfoGetSubDepartmentResp get_sub_department(token, base_info_get_sub_department_req)
获取子部门列表

获取子部门列表

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_get_sub_department_req** | Option<[**BaseInfoGetSubDepartmentReq**](BaseInfoGetSubDepartmentReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoGetSubDepartmentResp**](BaseInfoGetSubDepartmentResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tag_send_logs

> crate::models::BaseInfoGetTagSendLogsResp get_tag_send_logs(token, base_info_get_tag_send_logs_req)
获取发送历史记录

分页获取发送历史记录

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_get_tag_send_logs_req** | Option<[**BaseInfoGetTagSendLogsReq**](BaseInfoGetTagSendLogsReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoGetTagSendLogsResp**](BaseInfoGetTagSendLogsResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_friend_work_moments

> crate::models::BaseInfoGetUserFriendWorkMomentsResp get_user_friend_work_moments(token, base_info_get_user_friend_work_moments_req)
查询自己大工作圈页面

查询用户工作圈页面

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_get_user_friend_work_moments_req** | Option<[**BaseInfoGetUserFriendWorkMomentsReq**](BaseInfoGetUserFriendWorkMomentsReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoGetUserFriendWorkMomentsResp**](BaseInfoGetUserFriendWorkMomentsResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_in_department

> crate::models::BaseInfoGetUserInDepartmentResp get_user_in_department(token, base_info_get_user_in_department_req)
获取部门中的所有用户

获取部门中的所有用户

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_get_user_in_department_req** | Option<[**BaseInfoGetUserInDepartmentReq**](BaseInfoGetUserInDepartmentReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoGetUserInDepartmentResp**](BaseInfoGetUserInDepartmentResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_req_group_application_list

> crate::models::BaseInfoGetGroupApplicationListResp get_user_req_group_application_list(token, base_info_get_user_req_group_application_list_req)
获取用户加群申请列表

获取用户加群申请列表

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_get_user_req_group_application_list_req** | Option<[**BaseInfoGetUserReqGroupApplicationListReq**](BaseInfoGetUserReqGroupApplicationListReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoGetGroupApplicationListResp**](BaseInfoGetGroupApplicationListResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_tag_by_id

> crate::models::BaseInfoGetUserTagByIdResp get_user_tag_by_id(token, base_info_get_user_tag_by_id_req)
获取该用户的标签信息

通过标签id获取该用户的标签信息

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_get_user_tag_by_id_req** | Option<[**BaseInfoGetUserTagByIdReq**](BaseInfoGetUserTagByIdReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoGetUserTagByIdResp**](BaseInfoGetUserTagByIdResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_tags

> crate::models::BaseInfoGetUserTagsResp get_user_tags(token, base_info_get_user_tags_req)
获取用户标签信息

用户获取自己的所有的标签

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_get_user_tags_req** | Option<[**BaseInfoGetUserTagsReq**](BaseInfoGetUserTagsReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoGetUserTagsResp**](BaseInfoGetUserTagsResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_work_moments

> crate::models::BaseInfoGetUserWorkMomentsResp get_user_work_moments(token, base_info_get_user_work_moments_req)
查询用户工作圈

查询用户工作圈

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_get_user_work_moments_req** | Option<[**BaseInfoGetUserWorkMomentsReq**](BaseInfoGetUserWorkMomentsReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoGetUserWorkMomentsResp**](BaseInfoGetUserWorkMomentsResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users_info

> crate::models::BaseInfoGetUsersInfoResp get_users_info(token, base_info_get_users_info_req)
获取用户信息

根据用户列表批量获取用户信息

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_get_users_info_req** | Option<[**BaseInfoGetUsersInfoReq**](BaseInfoGetUsersInfoReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoGetUsersInfoResp**](BaseInfoGetUsersInfoResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users_online_status

> crate::models::BaseInfoGetUsersOnlineStatusResp get_users_online_status(token, base_info_get_users_online_status_req)
获取用户在线状态

获取用户在线状态

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_get_users_online_status_req** | Option<[**BaseInfoGetUsersOnlineStatusReq**](BaseInfoGetUsersOnlineStatusReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoGetUsersOnlineStatusResp**](BaseInfoGetUsersOnlineStatusResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_work_moment_by_id

> crate::models::BaseInfoGetWorkMomentByIdResp get_work_moment_by_id(token, base_info_get_work_moment_by_id_req)
通过ID获取工作圈

通过ID获取工作圈

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_get_work_moment_by_id_req** | Option<[**BaseInfoGetWorkMomentByIdReq**](BaseInfoGetWorkMomentByIdReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoGetWorkMomentByIdResp**](BaseInfoGetWorkMomentByIdResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## import_friend

> crate::models::BaseInfoImportFriendResp import_friend(token, base_info_import_friend_req)
批量加好友

批量加好友

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_import_friend_req** | Option<[**BaseInfoImportFriendReq**](BaseInfoImportFriendReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoImportFriendResp**](BaseInfoImportFriendResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## invite_user_to_group

> crate::models::BaseInfoInviteUserToGroupResp invite_user_to_group(token, base_info_invite_user_to_group_req)
将用户拉入群组

将用户拉入群组

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_invite_user_to_group_req** | Option<[**BaseInfoInviteUserToGroupReq**](BaseInfoInviteUserToGroupReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoInviteUserToGroupResp**](BaseInfoInviteUserToGroupResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## is_friend

> crate::models::BaseInfoIsFriendResp is_friend(token, base_info_is_friend_req)
检查用户之间是否为好友

检查用户之间是否为好友

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_is_friend_req** | Option<[**BaseInfoIsFriendReq**](BaseInfoIsFriendReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoIsFriendResp**](BaseInfoIsFriendResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## join_group

> crate::models::BaseInfoJoinGroupResp join_group(token, base_info_join_group_req)
加入群聊

加入群聊

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_join_group_req** | Option<[**BaseInfoJoinGroupReq**](BaseInfoJoinGroupReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoJoinGroupResp**](BaseInfoJoinGroupResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kick_group_member

> crate::models::BaseInfoKickGroupMemberResp kick_group_member(token, base_info_kick_group_member_req)
把用户踢出群组

把用户踢出群组

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_kick_group_member_req** | Option<[**BaseInfoKickGroupMemberReq**](BaseInfoKickGroupMemberReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoKickGroupMemberResp**](BaseInfoKickGroupMemberResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## like_one_work_moment

> crate::models::BaseInfoLikeOneWorkMomentResp like_one_work_moment(token, base_info_like_one_work_moment_req)
点赞一条工作圈

工作圈ID点赞一条工作圈

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_like_one_work_moment_req** | Option<[**BaseInfoLikeOneWorkMomentReq**](BaseInfoLikeOneWorkMomentReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoLikeOneWorkMomentResp**](BaseInfoLikeOneWorkMomentResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## management_batch_send_msg

> crate::models::BaseInfoManagementBatchSendMsgReq management_batch_send_msg(token, UNKNOWN_BASE_TYPE)
管理员批量发送群聊单聊消息

管理员批量发送群聊单聊消息 消息格式详细见<a href=\"https://doc.rentsoft.cn/#/server_doc/admin?id=%e6%b6%88%e6%81%af%e7%b1%bb%e5%9e%8b%e6%a0%bc%e5%bc%8f%e6%8f%8f%e8%bf%b0\">消息格式</href>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**UNKNOWN_BASE_TYPE** | Option<[**UNKNOWN_BASE_TYPE**](UNKNOWN_BASE_TYPE.md)> |  |  |

### Return type

[**crate::models::BaseInfoManagementBatchSendMsgReq**](BaseInfoManagementBatchSendMsgReq.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## management_send_msg

> crate::models::BaseInfoManagementSendMsgResp management_send_msg(token, UNKNOWN_BASE_TYPE)
管理员发送/撤回消息

管理员发送/撤回消息 消息格式详细见<a href=\"https://doc.rentsoft.cn/#/server_doc/admin?id=%e6%b6%88%e6%81%af%e7%b1%bb%e5%9e%8b%e6%a0%bc%e5%bc%8f%e6%8f%8f%e8%bf%b0\">消息格式</href>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**UNKNOWN_BASE_TYPE** | Option<[**UNKNOWN_BASE_TYPE**](UNKNOWN_BASE_TYPE.md)> |  |  |

### Return type

[**crate::models::BaseInfoManagementSendMsgResp**](BaseInfoManagementSendMsgResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## minio_upload_file

> crate::models::BaseInfoMinioUploadFileResp minio_upload_file(token, file, file_type, operation_id)
minio上传文件(web api)

minio上传文件(web api), 请注意本api请求为form并非json

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**file** | **std::path::PathBuf** | 要上传的文件文件 | [required] |
**file_type** | **i32** | 文件类型 | [required] |
**operation_id** | **String** | 操作唯一ID | [required] |

### Return type

[**crate::models::BaseInfoMinioUploadFileResp**](BaseInfoMinioUploadFileResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mute_group

> crate::models::BaseInfoMuteGroupResp mute_group(token, base_info_mute_group_req)
禁言群组

禁言群组

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_mute_group_req** | Option<[**BaseInfoMuteGroupReq**](BaseInfoMuteGroupReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoMuteGroupResp**](BaseInfoMuteGroupResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mute_group_member

> crate::models::BaseInfoDismissGroupResp mute_group_member(token, base_info_mute_group_member_req)
禁言群成员

禁言群成员

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_mute_group_member_req** | Option<[**BaseInfoMuteGroupMemberReq**](BaseInfoMuteGroupMemberReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoDismissGroupResp**](BaseInfoDismissGroupResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## parse_token

> crate::models::BaseInfoParseTokenResp parse_token(token, base_info_parse_token_req)
解析当前用户token

解析当前用户token(token在请求头中传入)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_parse_token_req** | Option<[**BaseInfoParseTokenReq**](BaseInfoParseTokenReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoParseTokenResp**](BaseInfoParseTokenResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## quit_group

> crate::models::BaseInfoQuitGroupResp quit_group(token, base_info_quit_group_req)
当前用户退出群聊

当前用户退出群聊

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_quit_group_req** | Option<[**BaseInfoQuitGroupReq**](BaseInfoQuitGroupReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoQuitGroupResp**](BaseInfoQuitGroupResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_black

> crate::models::BaseInfoRemoveBlackListResp remove_black(token, base_info_remove_black_list_req)
把用户移除黑名单

把用户移除黑名单

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_remove_black_list_req** | Option<[**BaseInfoRemoveBlackListReq**](BaseInfoRemoveBlackListReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoRemoveBlackListResp**](BaseInfoRemoveBlackListResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_msg2_tag

> crate::models::BaseInfoSendMsg2TagResp send_msg2_tag(token, base_info_send_msg2_tag_req)
发送标签消息

对标签用户发送消息

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_send_msg2_tag_req** | Option<[**BaseInfoSendMsg2TagReq**](BaseInfoSendMsg2TagReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoSendMsg2TagResp**](BaseInfoSendMsg2TagResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_friend_remark

> crate::models::BaseInfoSetFriendRemarkResp set_friend_remark(token, base_info_set_friend_remark_req)
设置好友备注

设置好友备注

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_set_friend_remark_req** | Option<[**BaseInfoSetFriendRemarkReq**](BaseInfoSetFriendRemarkReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoSetFriendRemarkResp**](BaseInfoSetFriendRemarkResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_global_recv_message_opt

> crate::models::BaseInfoSetGlobalRecvMessageOptResp set_global_recv_message_opt(token, base_info_set_global_recv_message_opt_req)
设置全局免打扰

设置全局免打扰

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_set_global_recv_message_opt_req** | Option<[**BaseInfoSetGlobalRecvMessageOptReq**](BaseInfoSetGlobalRecvMessageOptReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoSetGlobalRecvMessageOptResp**](BaseInfoSetGlobalRecvMessageOptResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_group_info

> crate::models::BaseInfoSetGroupInfoResp set_group_info(token, base_info_set_group_info_req)
设置群信息

设置群信息

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_set_group_info_req** | Option<[**BaseInfoSetGroupInfoReq**](BaseInfoSetGroupInfoReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoSetGroupInfoResp**](BaseInfoSetGroupInfoResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_group_member_info

> crate::models::BaseInfoSetGroupMemberInfoResp set_group_member_info(token, base_info_set_group_member_info_req)
修改群成员信息

修改群成员信息

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_set_group_member_info_req** | Option<[**BaseInfoSetGroupMemberInfoReq**](BaseInfoSetGroupMemberInfoReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoSetGroupMemberInfoResp**](BaseInfoSetGroupMemberInfoResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_tag

> crate::models::BaseInfoSetTagResp set_tag(token, base_info_set_tag_req)
修改标签

根据标签ID修改标签用户列表, 名称

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_set_tag_req** | Option<[**BaseInfoSetTagReq**](BaseInfoSetTagReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoSetTagResp**](BaseInfoSetTagResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transfer_group_owner

> crate::models::BaseInfoTransferGroupOwnerResp transfer_group_owner(token, base_info_transfer_group_owner_req)
转让群主

转让群主

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_transfer_group_owner_req** | Option<[**BaseInfoTransferGroupOwnerReq**](BaseInfoTransferGroupOwnerReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoTransferGroupOwnerResp**](BaseInfoTransferGroupOwnerResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_department

> crate::models::BaseInfoUpdateDepartmentResp update_department(token, base_info_update_department_req)
更新部门信息

更新部门信息

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_update_department_req** | Option<[**BaseInfoUpdateDepartmentReq**](BaseInfoUpdateDepartmentReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoUpdateDepartmentResp**](BaseInfoUpdateDepartmentResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_organization_user

> crate::models::BaseInfoUpdateOrganizationUserResp update_organization_user(token, base_info_update_organization_user_req)
更新组织架构中的用户

更新组织架构中的用户

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_update_organization_user_req** | Option<[**BaseInfoUpdateOrganizationUserReq**](BaseInfoUpdateOrganizationUserReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoUpdateOrganizationUserResp**](BaseInfoUpdateOrganizationUserResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_in_department

> crate::models::BaseInfoUpdateUserInDepartmentResp update_user_in_department(token, base_info_update_user_in_department_req)
更新部门中某个用户

更新部门中某个用户

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_update_user_in_department_req** | Option<[**BaseInfoUpdateUserInDepartmentReq**](BaseInfoUpdateUserInDepartmentReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoUpdateUserInDepartmentResp**](BaseInfoUpdateUserInDepartmentResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_info

> crate::models::BaseInfoUpdateUserInfoResp update_user_info(token, base_info_update_self_user_info_req)
修改用户信息

修改用户信息 userID faceURL等

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | im token | [required] |
**base_info_update_self_user_info_req** | Option<[**BaseInfoUpdateSelfUserInfoReq**](BaseInfoUpdateSelfUserInfoReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoUpdateUserInfoResp**](BaseInfoUpdateUserInfoResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_register

> crate::models::BaseInfoUserRegisterResp user_register(base_info_user_register_req)
用户注册

用户注册

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**base_info_user_register_req** | Option<[**BaseInfoUserRegisterReq**](BaseInfoUserRegisterReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoUserRegisterResp**](BaseInfoUserRegisterResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_token

> crate::models::BaseInfoUserTokenResp user_token(base_info_user_token_req)
用户登录

获取用户的token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**base_info_user_token_req** | Option<[**BaseInfoUserTokenReq**](BaseInfoUserTokenReq.md)> |  |  |

### Return type

[**crate::models::BaseInfoUserTokenResp**](BaseInfoUserTokenResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

