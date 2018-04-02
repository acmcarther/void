#[macro_use]
extern crate log;
extern crate vk_device_support as vkds;
extern crate vk_lite as vkl;
extern crate vk_swapchain_support as vkss;
extern crate vk_sys as vk;

use std::ffi::CString;
use std::ptr;
use std::mem;

pub struct PushConstantRangeGenerator {
  push_constant_ranges: Vec<vk::PushConstantRange>,
}

impl PushConstantRangeGenerator {
  pub fn new() -> PushConstantRangeGenerator {
    PushConstantRangeGenerator {
      push_constant_ranges: Vec::new(),
    }
  }

  /**
   * Enqueues a push constant type (T) as available with at the pipeline stages indicated by the
   * stage_flags.
   */
  pub fn push<'selff, T: Sized>(
    &'selff mut self,
    stage_flags: vk::PipelineStageFlags,
  ) -> &'selff mut PushConstantRangeGenerator {
    let existing_count = self.push_constant_ranges.len() as u32;
    self.push_constant_ranges.push(vk::PushConstantRange {
      stageFlags: stage_flags,
      offset: existing_count,
      size: std::mem::size_of::<T>() as u32,
    });

    self
  }

  pub fn take_ranges(&mut self) -> Vec<vk::PushConstantRange> {
    let mut swap_vec = Vec::new();
    mem::swap(&mut self.push_constant_ranges, &mut swap_vec);
    swap_vec
  }
}

/** Configures a render pass with one subpass and default fixed function pipeline settings. */
pub fn make_render_pass(
  device: &vkl::LDevice,
  depth_format: vk::Format,
  swapchain: &vkss::LoadedSwapchain,
) -> vkl::RawResult<vk::RenderPass> {
  let color_attachment_description = vk::AttachmentDescription {
    flags: 0,
    format: swapchain.surface_format.format,
    samples: vk::SAMPLE_COUNT_1_BIT,
    loadOp: vk::ATTACHMENT_LOAD_OP_CLEAR,
    storeOp: vk::ATTACHMENT_STORE_OP_STORE,
    stencilLoadOp: vk::ATTACHMENT_LOAD_OP_DONT_CARE,
    stencilStoreOp: vk::ATTACHMENT_STORE_OP_DONT_CARE,
    initialLayout: vk::IMAGE_LAYOUT_UNDEFINED,
    finalLayout: vk::IMAGE_LAYOUT_PRESENT_SRC_KHR,
  };

  let color_attachment_reference = vk::AttachmentReference {
    attachment: 0,
    layout: vk::IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL,
  };

  let depth_attachment_description = vk::AttachmentDescription {
    flags: 0,
    format: depth_format,
    samples: vk::SAMPLE_COUNT_1_BIT,
    loadOp: vk::ATTACHMENT_LOAD_OP_CLEAR,
    storeOp: vk::ATTACHMENT_STORE_OP_STORE,
    stencilLoadOp: vk::ATTACHMENT_LOAD_OP_DONT_CARE,
    stencilStoreOp: vk::ATTACHMENT_STORE_OP_DONT_CARE,
    initialLayout: vk::IMAGE_LAYOUT_UNDEFINED,
    finalLayout: vk::IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
  };

  let depth_attachment_reference = vk::AttachmentReference {
    attachment: 1,
    layout: vk::IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
  };

  let subpass_description = vk::SubpassDescription {
    flags: 0,
    pipelineBindPoint: vk::PIPELINE_BIND_POINT_GRAPHICS,
    inputAttachmentCount: 0,
    pInputAttachments: ptr::null(),
    colorAttachmentCount: 1,
    pColorAttachments: &color_attachment_reference,
    pResolveAttachments: ptr::null(),
    pDepthStencilAttachment: &depth_attachment_reference,
    preserveAttachmentCount: 0,
    pPreserveAttachments: ptr::null(),
  };

  let dependency = vk::SubpassDependency {
    srcSubpass: vk::SUBPASS_EXTERNAL,
    dstSubpass: 0,
    srcStageMask: vk::PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT,
    dstStageMask: vk::PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT,
    srcAccessMask: 0,
    dstAccessMask: vk::ACCESS_COLOR_ATTACHMENT_READ_BIT | vk::ACCESS_COLOR_ATTACHMENT_WRITE_BIT,
    dependencyFlags: 0,
  };

  info!("Vulkan creating render pass");

  let all_attachments = [color_attachment_description, depth_attachment_description];

  let render_pass_create_info = vk::RenderPassCreateInfo {
    sType: vk::STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO,
    pNext: ptr::null(),
    flags: 0,
    attachmentCount: all_attachments.len() as u32,
    pAttachments: all_attachments.as_ptr(),
    subpassCount: 1,
    pSubpasses: &subpass_description,
    dependencyCount: 1,
    pDependencies: &dependency,
  };

  device.create_render_pass(&render_pass_create_info)
}

pub fn make_pipeline_layout(
  device: &vkl::LDevice,
  descriptor_set_layouts: &Vec<vk::DescriptorSetLayout>,
  push_constant_ranges: &Vec<vk::PushConstantRange>,
) -> vkl::RawResult<vk::PipelineLayout> {
  let pipeline_layout_create_info = vk::PipelineLayoutCreateInfo {
    sType: vk::STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO,
    pNext: ptr::null(),
    flags: 0,
    setLayoutCount: descriptor_set_layouts.len() as u32,
    pSetLayouts: descriptor_set_layouts.as_ptr(),
    pushConstantRangeCount: push_constant_ranges.len() as u32,
    pPushConstantRanges: push_constant_ranges.as_ptr(),
  };

  device.create_pipeline_layout(&pipeline_layout_create_info)
}

pub fn make_graphics_pipeline(
  device: &vkl::LDevice,
  vert_shader_module: &vk::ShaderModule,
  frag_shader_module: &vk::ShaderModule,
  all_attr_desc: &Vec<vk::VertexInputAttributeDescription>,
  all_vertex_binding_desc: &Vec<vk::VertexInputBindingDescription>,
  render_pass: &vk::RenderPass,
  swapchain: &vkss::LoadedSwapchain,
  pipeline_layout: &vk::PipelineLayout,
) -> vkl::RawResult<vk::Pipeline> {
  let common_shader_pipeline_name = CString::new("main").unwrap();
  let pName = common_shader_pipeline_name.as_c_str().as_ptr();
  let vert_pipeline_shader_stage_create_info = vk::PipelineShaderStageCreateInfo {
    sType: vk::STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
    pNext: ptr::null(),
    flags: 0,
    stage: vk::SHADER_STAGE_VERTEX_BIT,
    module: *vert_shader_module,
    pName: pName,
    pSpecializationInfo: ptr::null(),
  };

  let frag_pipeline_shader_stage_create_info = vk::PipelineShaderStageCreateInfo {
    sType: vk::STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
    pNext: ptr::null(),
    flags: 0,
    stage: vk::SHADER_STAGE_FRAGMENT_BIT,
    module: *frag_shader_module,
    pName: pName,
    pSpecializationInfo: ptr::null(),
  };

  let pipeline_vertex_input_state_create_info = vk::PipelineVertexInputStateCreateInfo {
    sType: vk::STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
    pNext: ptr::null(),
    flags: 0,
    vertexBindingDescriptionCount: all_vertex_binding_desc.len() as u32,
    pVertexBindingDescriptions: all_vertex_binding_desc.as_ptr(),
    vertexAttributeDescriptionCount: all_attr_desc.len() as u32,
    pVertexAttributeDescriptions: all_attr_desc.as_ptr(),
  };

  let pipeline_input_assembly_state_create_info = vk::PipelineInputAssemblyStateCreateInfo {
    sType: vk::STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
    pNext: ptr::null(),
    flags: 0,
    topology: vk::PRIMITIVE_TOPOLOGY_TRIANGLE_LIST,
    primitiveRestartEnable: vk::FALSE,
  };

  let viewport = vk::Viewport {
    x: 0.0f32,
    y: 0.0f32,
    width: swapchain.surface_extent.width as f32,
    height: swapchain.surface_extent.height as f32,
    minDepth: 0.0f32,
    maxDepth: 1.0f32,
  };

  // Defines how the image in the viewport is truncated
  let scissor = vk::Rect2D {
    offset: vk::Offset2D { x: 0, y: 0 },
    extent: vk::Extent2D {
      width: swapchain.surface_extent.width,
      height: swapchain.surface_extent.height,
    },
  };

  let pipeline_viewport_state_create_info = vk::PipelineViewportStateCreateInfo {
    sType: vk::STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO,
    pNext: ptr::null(),
    flags: 0,
    viewportCount: 1,
    pViewports: &viewport,
    scissorCount: 1,
    pScissors: &scissor,
  };

  let pipeline_rasterization_state_create_info = vk::PipelineRasterizationStateCreateInfo {
    sType: vk::STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
    pNext: ptr::null(),
    flags: 0,
    depthClampEnable: vk::FALSE,
    rasterizerDiscardEnable: vk::FALSE,
    polygonMode: vk::POLYGON_MODE_FILL,
    cullMode: vk::CULL_MODE_BACK_BIT,
    frontFace: vk::FRONT_FACE_COUNTER_CLOCKWISE,
    depthBiasEnable: vk::FALSE,
    depthBiasConstantFactor: 0.0f32,
    depthBiasClamp: 0.0f32,
    depthBiasSlopeFactor: 0.0f32,
    lineWidth: 1.0f32,
  };

  // TODO(acmcarther): Examine these options
  // N.B: Enabling this requires a GPU extension.
  let pipeline_multisample_state_create_info = vk::PipelineMultisampleStateCreateInfo {
    sType: vk::STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
    pNext: ptr::null(),
    flags: 0,
    rasterizationSamples: vk::SAMPLE_COUNT_1_BIT,
    sampleShadingEnable: vk::FALSE,
    minSampleShading: 1.0f32,
    pSampleMask: ptr::null(),
    alphaToCoverageEnable: vk::FALSE,
    alphaToOneEnable: vk::FALSE,
  };

  // TODO(acmcarther): Depth and Stencil Testing
  // ...

  // TODO(acmcarther): Examine these options
  let pipeline_color_blend_attachment_state = vk::PipelineColorBlendAttachmentState {
    blendEnable: vk::FALSE,
    colorWriteMask: vk::COLOR_COMPONENT_R_BIT | vk::COLOR_COMPONENT_G_BIT
      | vk::COLOR_COMPONENT_B_BIT | vk::COLOR_COMPONENT_A_BIT,
    srcColorBlendFactor: vk::BLEND_FACTOR_ONE,
    dstColorBlendFactor: vk::BLEND_FACTOR_ZERO,
    colorBlendOp: vk::BLEND_OP_ADD,
    srcAlphaBlendFactor: vk::BLEND_FACTOR_ONE,
    dstAlphaBlendFactor: vk::BLEND_FACTOR_ZERO,
    alphaBlendOp: vk::BLEND_OP_ADD,
  };

  let pipeline_color_blend_state_create_info = vk::PipelineColorBlendStateCreateInfo {
    sType: vk::STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
    pNext: ptr::null(),
    flags: 0,
    logicOpEnable: vk::FALSE,
    logicOp: vk::LOGIC_OP_COPY,
    attachmentCount: 1,
    pAttachments: &pipeline_color_blend_attachment_state,
    blendConstants: [0f32, 0f32, 0f32, 0f32],
  };

  // TODO(acmcarther): This isn't properly configured
  let dynamic_states = [vk::DYNAMIC_STATE_VIEWPORT, vk::DYNAMIC_STATE_LINE_WIDTH];
  let pipeline_dynamic_state_create_info = vk::PipelineDynamicStateCreateInfo {
    sType: vk::STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO,
    pNext: ptr::null(),
    flags: 0,
    dynamicStateCount: 2,
    pDynamicStates: dynamic_states.as_ptr(),
  };

  let pipeline_shader_stage_infos = vec![
    vert_pipeline_shader_stage_create_info,
    frag_pipeline_shader_stage_create_info,
  ];

  let pipeline_depth_stencil_state_create_info = vk::PipelineDepthStencilStateCreateInfo {
    sType: vk::STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO,
    pNext: ptr::null(),
    flags: 0,
    depthTestEnable: vk::TRUE,
    depthWriteEnable: vk::TRUE,
    depthCompareOp: vk::COMPARE_OP_LESS,
    depthBoundsTestEnable: vk::FALSE,
    stencilTestEnable: vk::FALSE,
    front: vk::StencilOpState {
      failOp: 0,
      passOp: 0,
      depthFailOp: 0,
      compareOp: 0,
      compareMask: 0,
      writeMask: 0,
      reference: 0,
    },
    back: vk::StencilOpState {
      failOp: 0,
      passOp: 0,
      depthFailOp: 0,
      compareOp: 0,
      compareMask: 0,
      writeMask: 0,
      reference: 0,
    },
    minDepthBounds: 0.0f32,
    maxDepthBounds: 1.0f32,
  };

  let graphics_pipeline_create_info = vk::GraphicsPipelineCreateInfo {
    sType: vk::STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO,
    pNext: ptr::null(),
    flags: 0,
    stageCount: 2,
    pStages: pipeline_shader_stage_infos.as_ptr(),
    pVertexInputState: &pipeline_vertex_input_state_create_info,
    pInputAssemblyState: &pipeline_input_assembly_state_create_info,
    pTessellationState: ptr::null(),
    pViewportState: &pipeline_viewport_state_create_info,
    pRasterizationState: &pipeline_rasterization_state_create_info,
    pMultisampleState: &pipeline_multisample_state_create_info,
    pDepthStencilState: &pipeline_depth_stencil_state_create_info,
    pColorBlendState: &pipeline_color_blend_state_create_info,
    pDynamicState: ptr::null(),
    layout: *pipeline_layout,
    renderPass: *render_pass,
    subpass: 0,
    basePipelineHandle: 0, /* vk_null_handle */
    basePipelineIndex: -1,
  };

  device
    .create_graphics_pipelines(&vec![graphics_pipeline_create_info])
    .map(|mut r| r.remove(0))
}
