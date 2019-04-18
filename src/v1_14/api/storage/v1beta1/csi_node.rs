// Generated from definition io.k8s.api.storage.v1beta1.CSINode

/// CSINode holds information about all CSI drivers installed on a node. CSI drivers do not need to create the CSINode object directly. As long as they use the node-driver-registrar sidecar container, the kubelet will automatically populate the CSINode object for the CSI driver as part of kubelet plugin registration. CSINode has the same name as a node. If the object is missing, it means either there are no CSI Drivers available on the node, or the Kubelet version is low enough that it doesn't create this object. CSINode has an OwnerReference that points to the corresponding node object.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CSINode {
    /// metadata.name must be the Kubernetes node name.
    pub metadata: Option<crate::v1_14::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// spec is the specification of CSINode
    pub spec: crate::v1_14::api::storage::v1beta1::CSINodeSpec,
}

// Begin storage.k8s.io/v1beta1/CSINode

// Generated from operation createStorageV1beta1CSINode

impl CSINode {
    /// create a CSINode
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`CreateCSINodeResponse`]`>` constructor, or [`CreateCSINodeResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn create_csi_node(
        body: &crate::v1_14::api::storage::v1beta1::CSINode,
        optional: CreateCSINodeOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<CreateCSINodeResponse>), crate::RequestError> {
        let CreateCSINodeOptional {
            dry_run,
            field_manager,
            pretty,
        } = optional;
        let __url = "/apis/storage.k8s.io/v1beta1/csinodes?".to_string();
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
            __query_pairs.append_pair("dryRun", dry_run);
        }
        if let Some(field_manager) = field_manager {
            __query_pairs.append_pair("fieldManager", field_manager);
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::post(__url);
        let __body = serde_json::to_vec(&body).map_err(crate::RequestError::Json)?;
        match __request.body(__body) {
            Ok(body) => Ok((body, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`CSINode::create_csi_node`]
#[derive(Clone, Copy, Debug, Default)]
pub struct CreateCSINodeOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
    pub field_manager: Option<&'a str>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<CreateCSINodeResponse as Response>::try_from_parts` to parse the HTTP response body of [`CSINode::create_csi_node`]
#[derive(Debug)]
pub enum CreateCSINodeResponse {
    Ok(crate::v1_14::api::storage::v1beta1::CSINode),
    Created(crate::v1_14::api::storage::v1beta1::CSINode),
    Accepted(crate::v1_14::api::storage::v1beta1::CSINode),
    Unauthorized,
    Other,
}

impl crate::Response for CreateCSINodeResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateCSINodeResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateCSINodeResponse::Created(result), buf.len()))
            },
            http::StatusCode::ACCEPTED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateCSINodeResponse::Accepted(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((CreateCSINodeResponse::Unauthorized, 0)),
            _ => Ok((CreateCSINodeResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteStorageV1beta1CSINode

impl CSINode {
    /// delete a CSINode
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`DeleteCSINodeResponse`]`>` constructor, or [`DeleteCSINodeResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the CSINode
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn delete_csi_node(
        name: &str,
        optional: DeleteCSINodeOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<DeleteCSINodeResponse>), crate::RequestError> {
        let DeleteCSINodeOptional {
            dry_run,
            grace_period_seconds,
            orphan_dependents,
            pretty,
            propagation_policy,
        } = optional;
        let __url = format!("/apis/storage.k8s.io/v1beta1/csinodes/{name}?", name = name);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
            __query_pairs.append_pair("dryRun", dry_run);
        }
        if let Some(grace_period_seconds) = grace_period_seconds {
            __query_pairs.append_pair("gracePeriodSeconds", &grace_period_seconds.to_string());
        }
        if let Some(orphan_dependents) = orphan_dependents {
            __query_pairs.append_pair("orphanDependents", &orphan_dependents.to_string());
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        if let Some(propagation_policy) = propagation_policy {
            __query_pairs.append_pair("propagationPolicy", propagation_policy);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::delete(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(body) => Ok((body, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`CSINode::delete_csi_node`]
#[derive(Clone, Copy, Debug, Default)]
pub struct DeleteCSINodeOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately.
    pub grace_period_seconds: Option<i64>,
    /// Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the "orphan" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both.
    pub orphan_dependents: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy. Acceptable values are: 'Orphan' - orphan the dependents; 'Background' - allow the garbage collector to delete the dependents in the background; 'Foreground' - a cascading policy that deletes all dependents in the foreground.
    pub propagation_policy: Option<&'a str>,
}

/// Use `<DeleteCSINodeResponse as Response>::try_from_parts` to parse the HTTP response body of [`CSINode::delete_csi_node`]
#[derive(Debug)]
pub enum DeleteCSINodeResponse {
    OkStatus(crate::v1_14::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_14::api::storage::v1beta1::CSINode),
    Accepted(crate::v1_14::apimachinery::pkg::apis::meta::v1::Status),
    Unauthorized,
    Other,
}

impl crate::Response for DeleteCSINodeResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result: serde_json::Map<String, serde_json::Value> = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                let is_status = match result.get("kind") {
                    Some(serde_json::Value::String(s)) if s == "Status" => true,
                    _ => false,
                };
                if is_status {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteCSINodeResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteCSINodeResponse::OkValue(result), buf.len()))
                }
            },
            http::StatusCode::ACCEPTED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((DeleteCSINodeResponse::Accepted(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((DeleteCSINodeResponse::Unauthorized, 0)),
            _ => Ok((DeleteCSINodeResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteStorageV1beta1CollectionCSINode

impl CSINode {
    /// delete collection of CSINode
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`DeleteCollectionCSINodeResponse`]`>` constructor, or [`DeleteCollectionCSINodeResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn delete_collection_csi_node(
        optional: DeleteCollectionCSINodeOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<DeleteCollectionCSINodeResponse>), crate::RequestError> {
        let DeleteCollectionCSINodeOptional {
            continue_,
            field_selector,
            label_selector,
            limit,
            pretty,
            resource_version,
            timeout_seconds,
            watch,
        } = optional;
        let __url = "/apis/storage.k8s.io/v1beta1/csinodes?".to_string();
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(continue_) = continue_ {
            __query_pairs.append_pair("continue", continue_);
        }
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", field_selector);
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", label_selector);
        }
        if let Some(limit) = limit {
            __query_pairs.append_pair("limit", &limit.to_string());
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        if let Some(resource_version) = resource_version {
            __query_pairs.append_pair("resourceVersion", resource_version);
        }
        if let Some(timeout_seconds) = timeout_seconds {
            __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
        }
        if let Some(watch) = watch {
            __query_pairs.append_pair("watch", &watch.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::delete(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(body) => Ok((body, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`CSINode::delete_collection_csi_node`]
#[derive(Clone, Copy, Debug, Default)]
pub struct DeleteCollectionCSINodeOptional<'a> {
    /// The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server, the server will respond with a 410 ResourceExpired error together with a continue token. If the client needs a consistent list, it must restart their list without the continue field. Otherwise, the client may send another list request with the token received with the 410 error, the server will respond with a list starting from the next key, but from the latest snapshot, which is inconsistent from the previous list results - objects that are created, modified, or deleted after the first list request will be included in the response, as long as their keys are after the "next key".
    ///
    /// This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
    pub continue_: Option<&'a str>,
    /// A selector to restrict the list of returned objects by their fields. Defaults to everything.
    pub field_selector: Option<&'a str>,
    /// A selector to restrict the list of returned objects by their labels. Defaults to everything.
    pub label_selector: Option<&'a str>,
    /// limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.
    ///
    /// The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
    pub limit: Option<i64>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
    pub resource_version: Option<&'a str>,
    /// Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity.
    pub timeout_seconds: Option<i64>,
    /// Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub watch: Option<bool>,
}

/// Use `<DeleteCollectionCSINodeResponse as Response>::try_from_parts` to parse the HTTP response body of [`CSINode::delete_collection_csi_node`]
#[derive(Debug)]
pub enum DeleteCollectionCSINodeResponse {
    OkStatus(crate::v1_14::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_14::api::storage::v1beta1::CSINode),
    Unauthorized,
    Other,
}

impl crate::Response for DeleteCollectionCSINodeResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result: serde_json::Map<String, serde_json::Value> = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                let is_status = match result.get("kind") {
                    Some(serde_json::Value::String(s)) if s == "Status" => true,
                    _ => false,
                };
                if is_status {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteCollectionCSINodeResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteCollectionCSINodeResponse::OkValue(result), buf.len()))
                }
            },
            http::StatusCode::UNAUTHORIZED => Ok((DeleteCollectionCSINodeResponse::Unauthorized, 0)),
            _ => Ok((DeleteCollectionCSINodeResponse::Other, 0)),
        }
    }
}

// Generated from operation listStorageV1beta1CSINode

impl CSINode {
    /// list or watch objects of kind CSINode
    ///
    /// This operation only supports listing all items of this type.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ListCSINodeResponse`]`>` constructor, or [`ListCSINodeResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn list_csi_node(
        optional: crate::v1_14::ListOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ListCSINodeResponse>), crate::RequestError> {
        let crate::v1_14::ListOptional {
            continue_,
            field_selector,
            label_selector,
            limit,
            pretty,
            resource_version,
            timeout_seconds,
        } = optional;
        let __url = "/apis/storage.k8s.io/v1beta1/csinodes?".to_string();
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(continue_) = continue_ {
            __query_pairs.append_pair("continue", continue_);
        }
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", field_selector);
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", label_selector);
        }
        if let Some(limit) = limit {
            __query_pairs.append_pair("limit", &limit.to_string());
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        if let Some(resource_version) = resource_version {
            __query_pairs.append_pair("resourceVersion", resource_version);
        }
        if let Some(timeout_seconds) = timeout_seconds {
            __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(body) => Ok((body, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Use `<ListCSINodeResponse as Response>::try_from_parts` to parse the HTTP response body of [`CSINode::list_csi_node`]
#[derive(Debug)]
pub enum ListCSINodeResponse {
    Ok(crate::v1_14::api::storage::v1beta1::CSINodeList),
    Unauthorized,
    Other,
}

impl crate::Response for ListCSINodeResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ListCSINodeResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ListCSINodeResponse::Unauthorized, 0)),
            _ => Ok((ListCSINodeResponse::Other, 0)),
        }
    }
}

// Generated from operation patchStorageV1beta1CSINode

impl CSINode {
    /// partially update the specified CSINode
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`PatchCSINodeResponse`]`>` constructor, or [`PatchCSINodeResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the CSINode
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn patch_csi_node(
        name: &str,
        body: &crate::v1_14::apimachinery::pkg::apis::meta::v1::Patch,
        optional: PatchCSINodeOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<PatchCSINodeResponse>), crate::RequestError> {
        let PatchCSINodeOptional {
            dry_run,
            field_manager,
            force,
            pretty,
        } = optional;
        let __url = format!("/apis/storage.k8s.io/v1beta1/csinodes/{name}?", name = name);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
            __query_pairs.append_pair("dryRun", dry_run);
        }
        if let Some(field_manager) = field_manager {
            __query_pairs.append_pair("fieldManager", field_manager);
        }
        if let Some(force) = force {
            __query_pairs.append_pair("force", &force.to_string());
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::patch(__url);
        let __body = serde_json::to_vec(&body).map_err(crate::RequestError::Json)?;
        match __request.body(__body) {
            Ok(body) => Ok((body, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`CSINode::patch_csi_node`]
#[derive(Clone, Copy, Debug, Default)]
pub struct PatchCSINodeOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. This field is required for apply requests (application/apply-patch) but optional for non-apply patch types (JsonPatch, MergePatch, StrategicMergePatch).
    pub field_manager: Option<&'a str>,
    /// Force is going to "force" Apply requests. It means user will re-acquire conflicting fields owned by other people. Force flag must be unset for non-apply patch requests.
    pub force: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<PatchCSINodeResponse as Response>::try_from_parts` to parse the HTTP response body of [`CSINode::patch_csi_node`]
#[derive(Debug)]
pub enum PatchCSINodeResponse {
    Ok(crate::v1_14::api::storage::v1beta1::CSINode),
    Unauthorized,
    Other,
}

impl crate::Response for PatchCSINodeResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchCSINodeResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((PatchCSINodeResponse::Unauthorized, 0)),
            _ => Ok((PatchCSINodeResponse::Other, 0)),
        }
    }
}

// Generated from operation readStorageV1beta1CSINode

impl CSINode {
    /// read the specified CSINode
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReadCSINodeResponse`]`>` constructor, or [`ReadCSINodeResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the CSINode
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn read_csi_node(
        name: &str,
        optional: ReadCSINodeOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReadCSINodeResponse>), crate::RequestError> {
        let ReadCSINodeOptional {
            exact,
            export,
            pretty,
        } = optional;
        let __url = format!("/apis/storage.k8s.io/v1beta1/csinodes/{name}?", name = name);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(exact) = exact {
            __query_pairs.append_pair("exact", &exact.to_string());
        }
        if let Some(export) = export {
            __query_pairs.append_pair("export", &export.to_string());
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(body) => Ok((body, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`CSINode::read_csi_node`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReadCSINodeOptional<'a> {
    /// Should the export be exact.  Exact export maintains cluster-specific fields like 'Namespace'. Deprecated. Planned for removal in 1.18.
    pub exact: Option<bool>,
    /// Should this value be exported.  Export strips fields that a user can not specify. Deprecated. Planned for removal in 1.18.
    pub export: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReadCSINodeResponse as Response>::try_from_parts` to parse the HTTP response body of [`CSINode::read_csi_node`]
#[derive(Debug)]
pub enum ReadCSINodeResponse {
    Ok(crate::v1_14::api::storage::v1beta1::CSINode),
    Unauthorized,
    Other,
}

impl crate::Response for ReadCSINodeResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadCSINodeResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReadCSINodeResponse::Unauthorized, 0)),
            _ => Ok((ReadCSINodeResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceStorageV1beta1CSINode

impl CSINode {
    /// replace the specified CSINode
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReplaceCSINodeResponse`]`>` constructor, or [`ReplaceCSINodeResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the CSINode
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn replace_csi_node(
        name: &str,
        body: &crate::v1_14::api::storage::v1beta1::CSINode,
        optional: ReplaceCSINodeOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<ReplaceCSINodeResponse>), crate::RequestError> {
        let ReplaceCSINodeOptional {
            dry_run,
            field_manager,
            pretty,
        } = optional;
        let __url = format!("/apis/storage.k8s.io/v1beta1/csinodes/{name}?", name = name);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
            __query_pairs.append_pair("dryRun", dry_run);
        }
        if let Some(field_manager) = field_manager {
            __query_pairs.append_pair("fieldManager", field_manager);
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::put(__url);
        let __body = serde_json::to_vec(&body).map_err(crate::RequestError::Json)?;
        match __request.body(__body) {
            Ok(body) => Ok((body, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`CSINode::replace_csi_node`]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReplaceCSINodeOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
    pub field_manager: Option<&'a str>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReplaceCSINodeResponse as Response>::try_from_parts` to parse the HTTP response body of [`CSINode::replace_csi_node`]
#[derive(Debug)]
pub enum ReplaceCSINodeResponse {
    Ok(crate::v1_14::api::storage::v1beta1::CSINode),
    Created(crate::v1_14::api::storage::v1beta1::CSINode),
    Unauthorized,
    Other,
}

impl crate::Response for ReplaceCSINodeResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceCSINodeResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceCSINodeResponse::Created(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReplaceCSINodeResponse::Unauthorized, 0)),
            _ => Ok((ReplaceCSINodeResponse::Other, 0)),
        }
    }
}

// Generated from operation watchStorageV1beta1CSINode

impl CSINode {
    /// list or watch objects of kind CSINode
    ///
    /// This operation only supports watching one item, or a list of items, of this type for changes.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`WatchCSINodeResponse`]`>` constructor, or [`WatchCSINodeResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn watch_csi_node(
        optional: crate::v1_14::WatchOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<WatchCSINodeResponse>), crate::RequestError> {
        let crate::v1_14::WatchOptional {
            field_selector,
            label_selector,
            pretty,
            resource_version,
            timeout_seconds,
        } = optional;
        let __url = "/apis/storage.k8s.io/v1beta1/csinodes?".to_string();
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", field_selector);
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", label_selector);
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        if let Some(resource_version) = resource_version {
            __query_pairs.append_pair("resourceVersion", resource_version);
        }
        if let Some(timeout_seconds) = timeout_seconds {
            __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
        }
        __query_pairs.append_pair("watch", "true");
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(body) => Ok((body, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Use `<WatchCSINodeResponse as Response>::try_from_parts` to parse the HTTP response body of [`CSINode::watch_csi_node`]
#[derive(Debug)]
pub enum WatchCSINodeResponse {
    Ok(crate::v1_14::apimachinery::pkg::apis::meta::v1::WatchEvent<CSINode>),
    Unauthorized,
    Other,
}

impl crate::Response for WatchCSINodeResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let mut deserializer = serde_json::Deserializer::from_slice(buf).into_iter();
                let (result, byte_offset) = match deserializer.next() {
                    Some(Ok(value)) => (value, deserializer.byte_offset()),
                    Some(Err(ref err)) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Some(Err(err)) => return Err(crate::ResponseError::Json(err)),
                    None => return Err(crate::ResponseError::NeedMoreData),
                };
                Ok((WatchCSINodeResponse::Ok(result), byte_offset))
            },
            http::StatusCode::UNAUTHORIZED => Ok((WatchCSINodeResponse::Unauthorized, 0)),
            _ => Ok((WatchCSINodeResponse::Other, 0)),
        }
    }
}

// End storage.k8s.io/v1beta1/CSINode

impl crate::Resource for CSINode {
    fn api_version() -> &'static str {
        "storage.k8s.io/v1beta1"
    }

    fn group() -> &'static str {
        "storage.k8s.io"
    }

    fn kind() -> &'static str {
        "CSINode"
    }

    fn version() -> &'static str {
        "v1beta1"
    }
}

impl crate::Metadata for CSINode {
    type Ty = crate::v1_14::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> Option<&<Self as crate::Metadata>::Ty> {
        self.metadata.as_ref()
    }
}

impl<'de> serde::Deserialize<'de> for CSINode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_metadata,
            Key_spec,
            Other,
        }

        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(f, "field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
                        Ok(match v {
                            "apiVersion" => Field::Key_api_version,
                            "kind" => Field::Key_kind,
                            "metadata" => Field::Key_metadata,
                            "spec" => Field::Key_spec,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CSINode;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "struct CSINode")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_metadata: Option<crate::v1_14::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<crate::v1_14::api::storage::v1beta1::CSINodeSpec> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => {
                            let value_api_version: String = serde::de::MapAccess::next_value(&mut map)?;
                            if value_api_version != <Self::Value as crate::Resource>::api_version() {
                                return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(&value_api_version), &<Self::Value as crate::Resource>::api_version()));
                            }
                        },
                        Field::Key_kind => {
                            let value_kind: String = serde::de::MapAccess::next_value(&mut map)?;
                            if value_kind != <Self::Value as crate::Resource>::kind() {
                                return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(&value_kind), &<Self::Value as crate::Resource>::kind()));
                            }
                        },
                        Field::Key_metadata => value_metadata = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_spec => value_spec = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CSINode {
                    metadata: value_metadata,
                    spec: value_spec.ok_or_else(|| serde::de::Error::missing_field("spec"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "CSINode",
            &[
                "apiVersion",
                "kind",
                "metadata",
                "spec",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for CSINode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CSINode",
            3 +
            self.metadata.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as crate::Resource>::api_version())?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as crate::Resource>::kind())?;
        if let Some(value) = &self.metadata {
            serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "spec", &self.spec)?;
        serde::ser::SerializeStruct::end(state)
    }
}
