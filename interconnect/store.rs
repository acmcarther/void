extern crate interconnect;
extern crate futures;
extern crate lazy_static;
extern crate log;
extern crate zcfg;
extern crate init;

pub type StoreResult<T> = Result<T, StoreErr>;

enum StoreError {
  Retriable(RetriableError)
  Serious(SeriousError)
}

enum RetriableError {
  message: String,
}

enum SeriousError {
  message: String,
}

struct ReserveEntityIdsResponse {
  ids: Vec<u64>,
  error: Option<ReserveEntityIdsError>
}

struct ReserveEntityIdsError {
  unreserved_id_count: u32,
  error: StoreErr,
}

struct CreateEntitiesRequest {
  ids: Vec<u64>,
}

struct CreateEntityErr {
  id: u64,
  error: StoreErr,
}

struct CreateEntitiesResponse {
  ids: Vec<u64>,
  uncreated_ids: Vec<CreateEntityErr>
}

struct DeleteEntitiesRequest {
  ids: Vec<u64>,
}

struct DeleteEntityErr {
  id: u64,
  error: StoreErr,
}

struct DeleteEntitiesResponse {
  ids: Vec<u64>,
  undeleted_ids: Vec<DeleteEntityErr>
}

trait StateStore {
  fn reserve_entity_ids(&self, request: ReserveEntityIdsRequest) -> ReserveEntityIdsResponse;
  fn create_entities(&self, request: CreateEntitiesRequest) -> CreateEntitiesResponse;
  fn delete_entities(&self, request: DeleteEntitiesRequest) -> DeleteEntitiesResponse;
  fn add_physics_component(&self, request: DeleteEntitiesRequest) -> DeleteEntitiesResponse;
}
