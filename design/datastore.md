Chunk: Segment of EntityId - ComponentTable entries
Chunkman/Chunkmen: Owns segments of chunks, operating in a consensus group
ComponentTable: A chunked proto-like object, keyed on EntityIdo

Per tick, ComponentTable deltas are emitted to a component tick table

```
table_decl {
  data_type: "something.something.MyData"
}

caching: {
  AGGRESSIVE,
  RESPONSIVE,
  NEVER,
}

overload_behavior: {
  TIME_DILATION,
  DROP_TICK,
}
```

```
type EntityId: u64;
type TransactionId: u64;

struct LockMetadata {
  lock_duration_micros: u64,
}

struct ReadTransaction {
  lock_metadata: LockMetadata,
}

struct ReadWriteTransaction {
  lock_metadata: LockMetadata,
}

struct EntityPartition {
  begin: u64,
  end: u64,
}

struct ReadResult<D> {
  data: Option<D>,
}

struct WriteResult {
}

trait ChunkApi<D> {
  fn read_entity(id: EntityId) -> ReadResult<D>;
  fn write_entity(id: EntityId, data: D) -> WriteResult;
}
```
