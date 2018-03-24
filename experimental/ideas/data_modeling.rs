type NodeId = u64;
type ServiceId = u64;
type Address = String;

struct NodeRegistry {
  node_to_addr: HashMap<NodeId, Address>,
}

struct ServiceIdent {
  node_id: Option<NodeId>,
  service_id: ServiceId,
}

struct ParitionedDataSpec {
  owner: Option<ServiceIdent>,
}

struct Vec3f {
  x: f32,
  y: f32,
  z: f32,
}

struct PhysicsData {
  pos: Vec3f,
  vel: Vec3f,
  mass: f32,
}

/**
 * Types of core data
 *
 * Global data:
 *   Relatively small or infrequently updated state that should be accessible from anywhere in
 * the universe.
 * Examples:
 * - Connection state
 * - Chat state
 * - Stats
 * - Most things that are not core to a "real time experience"
 *
 *
 * Regional data:
 * Data whose accessibility is limited to some partitionable space, and can therefore be lazily
 * (> 1 tick) replicated globally.
 * - Object position data
 * - Circuit state
 */

/**
 * State (in order of durability, speed of access, latency)
 * - SystemLocal (not client accessible)
 * - SystemPersistent (optional) (not client accessible)
 * - Regional
 * - RegionalGateway
 * - Global
 * - GlobalGateway
 * - Snapshot
 */

/**
 * Pathological cases
 *
 * Overloaded Global System: E.g. chat system overloaded
 * - Shard system logic regionally with eventual consistency
 *
 * Overloaded Regional System
 * - Subdivide region
 * - TODO how to reshard?
 *
 * Too small regions (amount of speculative work outside of region exceeds a threshold)
 * - Rebalance regions
 */

/**
 * Resharding state
 *
 * Fragmenting regional state
 * - Add subregional master elections
 * - Force mastership change over region into subregion (after lease expires)
 *
 * Merging Regional state
 * - Force mastership change for two subregions into parent region 
 * - Remove sub-regional master election
 */

/**
 * Client server architecture
 *
 * Clients receive their data from a Global gateway, and regional gateways as appropriate. In a
 * pinch, all state can be read from the global gateway, but RTT will be suboptimal as there are no
 * guarantess on propagation speed for Global gateway.
 */

/**
 * Lifecycle of a physics tick
 *
 * 1) A physics node identifies its working area
 * 2) It queries for updates in its working area, and "nearby"
 * 3) It runs a physics tick for its working area, updating local state
 * 4) It pushes the update upstream.
 */

/**
 * Lifecycle of an electronics tick
 *
 * 1) An electronics node identifies its working area (set of circuits)
 * 2) It runs an electronics tick for its working area, updating local state
 * 3) It pushes the update upstream
 */

/**
 * Lifecycle of a debris tick
 *
 * 1) A debris node queries for latest brittle collision events
 * 2) For each brittle collision event, it transactionally creates a debris object at collision site
 */

/**
 * Lifecycle of a player input manager tick
 * TODO: revisit
 *
 * 1) A player manager queries for latest input events
 * 2) It pulls latest real player state
 * 3) It updates local optimistic player state
 * 4) For each player input event, it asynchronously + transactionally updates player state
 * 5) Concurrently it updates optimistic player state
 * 6) It pushes optimistic player state
 */

/**
 * Lifecycle of a particle system tick
 * TODO: revisit
 *
 * 1) A particle system listens for particle emitter events
 * 2) It updates local particle emitter state
 * 3) It pushes the update upstream
 */
