// This tutorial demonstrates how to implement a thread-safe, generic Least Recently Used (LRU) cache from scratch in Go.
// You will learn about:
// 1. Go's generics: How to write type-agnostic code using type parameters for keys and values.
// 2. Concurrency with mutexes: Protecting shared data structures from race conditions using sync.Mutex.
// 3. LRU Cache algorithm: The core logic of storing and evicting items based on recency of use.
//
// An LRU cache combines a hash map (for O(1) average time lookups) with a doubly linked list
// (for O(1) recency updates and O(1) eviction of the least recently used item).

import (
	"sync" // The sync package provides synchronization primitives like Mutex.
)

// cacheEntry represents an entry in the LRU cache's doubly linked list.
// It stores the key, value, and pointers to the previous and next entries.
// K is the type of the key, which must be 'comparable' (e.g., int, string, structs that support == and !=).
// V is the type of the value, which can be 'any' type.
type cacheEntry[K comparable, V any] struct {
	key   K
	value V
	prev  *cacheEntry[K, V] // Pointer to the previous entry in the list.
	next  *cacheEntry[K, V] // Pointer to the next entry in the list.
}

// LRUCache implements the LRU cache functionality.
// It uses a map for fast key lookups and a doubly linked list to maintain
// the order of recency (most recently used at the front, least recently used at the back).
type LRUCache[K comparable, V any] struct {
	capacity int                           // Maximum number of items the cache can hold.
	size     int                           // Current number of items in the cache.
	cacheMap map[K]*cacheEntry[K, V]       // Maps keys to their corresponding cache entries for O(1) lookup.
	head     *cacheEntry[K, V]             // Pointer to the most recently used entry (front of the list).
	tail     *cacheEntry[K, V]             // Pointer to the least recently used entry (back of the list).
	mutex    sync.Mutex                    // A mutex to protect all shared data (cacheMap, head, tail, size) from concurrent access.
}

// NewLRUCache creates and initializes a new LRUCache with the given capacity.
// It uses generic type parameters K and V, making the cache reusable for any comparable key type
// and any value type.
func NewLRUCache[K comparable, V any](capacity int) *LRUCache[K, V] {
	if capacity <= 0 {
		panic("Cache capacity must be greater than 0") // An LRU cache needs a positive capacity to function.
	}
	return &LRUCache[K, V]{
		capacity: capacity,
		cacheMap: make(map[K]*cacheEntry[K, V]), // Initialize the underlying map.
	}
}

// Get retrieves a value from the cache.
// If the key exists, it returns the value and true, also marking the entry
// as most recently used by moving it to the front of the list.
// Otherwise, it returns the zero value of V and false.
func (c *LRUCache[K, V]) Get(key K) (V, bool) {
	c.mutex.Lock()         // Acquire the lock to ensure thread safety before accessing shared data.
	defer c.mutex.Unlock() // Release the lock when the function exits, guaranteeing it's always unlocked.

	if entry, found := c.cacheMap[key]; found {
		c.moveToFront(entry) // This entry was just accessed, so it's now the MRU item.
		return entry.value, true
	}
	var zeroValue V // Declare a variable of type V to get its zero value.
	return zeroValue, false
}

// Put adds or updates a value in the cache.
// If the key already exists, its value is updated, and it's moved to the front (MRU).
// If the key is new, it's added to the front. If the cache is already at capacity,
// the least recently used item (at the tail) is removed first to make space.
func (c *LRUCache[K, V]) Put(key K, value V) {
	c.mutex.Lock()         // Acquire the lock for thread safety.
	defer c.mutex.Unlock() // Release the lock.

	if entry, found := c.cacheMap[key]; found {
		// Key already exists: update its value and move it to the front (MRU).
		entry.value = value
		c.moveToFront(entry)
		return
	}

	// Key does not exist: create a new entry.
	newEntry := &cacheEntry[K, V]{key: key, value: value}
	c.cacheMap[key] = newEntry // Add the new entry to the map for quick lookups.
	c.addFront(newEntry)       // Add the new entry to the front of the list (it's the new MRU).
	c.size++                   // Increment the cache's current size.

	// Check if the cache has exceeded its capacity.
	if c.size > c.capacity {
		// Capacity exceeded: remove the least recently used item (from the tail).
		c.removeTail()
	}
}

// --- Doubly Linked List Helper Functions (internal to the LRUCache logic) ---

// moveToFront moves an existing entry to the front of the doubly linked list.
// This signifies it has become the most recently used item.
func (c *LRUCache[K, V]) moveToFront(entry *cacheEntry[K, V]) {
	if entry == c.head {
		return // Already at the front, no action needed.
	}
	c.remove(entry)   // First, remove the entry from its current position.
	c.addFront(entry) // Then, add it to the front of the list.
}

// remove removes an entry from the doubly linked list.
// It handles cases where the entry is the head, tail, or in the middle.
func (c *LRUCache[K, V]) remove(entry *cacheEntry[K, V]) {
	if entry.prev != nil {
		entry.prev.next = entry.next // Connect the previous entry to the next entry.
	} else {
		c.head = entry.next // If 'entry' was the head, its next element becomes the new head.
	}

	if entry.next != nil {
		entry.next.prev = entry.prev // Connect the next entry to the previous entry.
	} else {
		c.tail = entry.prev // If 'entry' was the tail, its previous element becomes the new tail.
	}
	// Clear pointers to indicate the entry is no longer part of the list.
	entry.next = nil
	entry.prev = nil
}

// addFront adds a new entry to the front (head) of the doubly linked list.
func (c *LRUCache[K, V]) addFront(entry *cacheEntry[K, V]) {
	entry.next = c.head // The new entry's 'next' pointer points to the current head.
	entry.prev = nil    // The new entry has no previous element as it's the new head.

	if c.head != nil {
		c.head.prev = entry // If there was an old head, its 'prev' pointer now points to the new entry.
	}
	c.head = entry // Update the cache's head pointer to the new entry.

	if c.tail == nil {
		c.tail = entry // If the list was empty, the new entry is also the tail.
	}
}

// removeTail removes the least recently used entry (the one at the tail) from the cache.
// This is called when the cache capacity is exceeded, performing the LRU eviction.
func (c *LRUCache[K, V]) removeTail() {
	if c.tail == nil {
		return // Nothing to remove if the cache is empty.
	}
	oldTailKey := c.tail.key // Store the key of the tail entry before removal.
	c.remove(c.tail)         // Remove the tail entry from the linked list.
	delete(c.cacheMap, oldTailKey) // Remove the entry from the map using its key.
	c.size--                 // Decrement the cache's current size.
}


// --- Example Usage ---
// This main function demonstrates how to use the generic, thread-safe LRU cache.
func main() {
	// Create a new LRU cache that stores strings as keys and ints as values, with a capacity of 3.
	cache := NewLRUCache[string, int](3)
	println("LRU Cache initialized with Capacity: 3")

	// 1. Add items up to capacity.
	cache.Put("a", 1) // Cache state: [a]
	cache.Put("b", 2) // Cache state: [b, a]
	cache.Put("c", 3) // Cache state: [c, b, a] (Cache is now full)
	println("Put: a=1, b=2, c=3. Cache is now full.")

	// 2. Access an item - it should become the Most Recently Used (MRU).
	if val, found := cache.Get("b"); found {
		println("Get 'b':", val, "(now MRU)") // Output: Get 'b': 2
	}
	// Cache state after Get("b"): [b, c, a] ('a' is now LRU)

	// 3. Put a new item when cache is full - triggers eviction of the LRU item.
	cache.Put("d", 4) // Cache state: [d, b, c] ('a' was LRU and is now evicted)
	println("Put: d=4. 'a' (LRU) should be evicted.")

	// 4. Try to retrieve the evicted item.
	if _, found := cache.Get("a"); !found {
		println("Get 'a': Not found (as expected, 'a' was evicted)") // Output: Get 'a': Not found
	}

	// 5. Access another item to show MRU update again.
	if val, found := cache.Get("c"); found {
		println("Get 'c':", val, "(now MRU)") // Output: Get 'c': 3
	}
	// Cache state after Get("c"): [c, d, b] ('b' is now LRU)

	// 6. Put another new item - triggers eviction of the new LRU item.
	cache.Put("e", 5) // Cache state: [e, c, d] ('b' was LRU and is now evicted)
	println("Put: e=5. 'b' (LRU) should be evicted.")
}