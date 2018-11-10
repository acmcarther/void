# VK planning

Api composed of independent manager objects with lifetimes derived from each other.


VulkanLib: Raw dylib that exposes all contained function pointers.

VkInstance: An instance object derived from a vulkan lib with lifetime correlated to the vulkan lib

VkDevice
