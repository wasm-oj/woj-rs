# \SubmissionApi

All URIs are relative to *https://woj.csie.cool*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_submission**](SubmissionApi.md#get_submission) | **GET** /api/submission/{id} | 
[**list_submissions**](SubmissionApi.md#list_submissions) | **GET** /api/submission | 
[**submit**](SubmissionApi.md#submit) | **POST** /api/submission | 



## get_submission

> crate::models::GetSubmission200Response get_submission(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::GetSubmission200Response**](get_submission_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_submissions

> crate::models::ListSubmissions200Response list_submissions(page, lang, problem, status, submitter)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**String**> |  |  |[default to 1]
**lang** | Option<**String**> |  |  |
**problem** | Option<**String**> |  |  |
**status** | Option<**String**> |  |  |
**submitter** | Option<**String**> |  |  |

### Return type

[**crate::models::ListSubmissions200Response**](list_submissions_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submit

> crate::models::Submit200Response submit(submit_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**submit_request** | Option<[**SubmitRequest**](SubmitRequest.md)> |  |  |

### Return type

[**crate::models::Submit200Response**](submit_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

