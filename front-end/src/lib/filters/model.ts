export let model_filters = {
  period: {
    selected: {name: "Month", value: "Month"},
    options: [
      {name: "Day", value: "Day"},
      {name: "Week", value: "Week"},
      {name: "Month", value: "Month"},
      {name: "Year", value: "Year"},
      {name: "All Time", value: "AllTime"}
    ]
  },
  sort: {
    selected: {name: "Highest Rated", value: "Highest Rated"},
    options: [
      {name: "Highest Rated", value: "Highest Rated" },
      {name: "Most Downloaded", value: "Most Downloaded" },
      { name: "Most Liked", value: "Most Liked" },
      { name: "Most Discussed", value: "Most Discussed" },
      { name: "Most Collected", value: "Most Collected" },
      { name: "Most Images", value: "Most Images" },
      { name: "Newest", value: "Newest" },
      {name: "Oldest", value: "Oldest"},
    ]
  },
  browsingLevel: { // NSFW Filter
    selected: { name: "Include Nsfw", value: "31" },
    options: [
      { name: "Hide Nsfw", value: "0" },
      { name: "Include Nsfw", value: "31" },
      { name: "Only Nsfw", value: "100" },
    ]
  },
  baseModel: {
    selected: { name: "", value: "" },
    options: []
  },
  types: {
    selected: { name: "", value: "" },
    options: [
      create_option_from_v("Checkpoint"),
      create_option_from_v("TextualInversion"),
      create_option_from_v("Hypernetwork"),
      create_option_from_v("AestheticGradient"),
      create_option_from_v("MotionModule"),
      create_option_from_v("Upscaler"),
      create_option_from_v("Controlnet"),
      create_option_from_v("DoRA"),
      create_option_from_v("LoCon"),
      create_option_from_v("LORA"),
      create_option_from_v("VAE"),
      create_option_from_v("Poses"),
      create_option_from_v("Wildcards"),
      create_option_from_v("Workflows"),
      create_option_from_v("Detection"),
      create_option_from_v("Other"),
    ]
  },
  earlyAccess: generate_boolean_type(),
  supportsGeneration: generate_boolean_type(),
  fromPlatform: generate_boolean_type(),
  isFeatured: generate_boolean_type(),
  checkpointType: {
    selected: create_option_from_v("All"),
    options: [
      create_option_from_v("All"),
      create_option_from_v("Trained"),
      create_option_from_v("Merge")
    ]
  },
  fileFormat: {
    selected: create_option_from_v(""),
    options: [
      create_option_from_v("ONNX"),
      create_option_from_v("SafeTensor"),
      create_option_from_v("PickleTensor"),
      create_option_from_v("GGUF"),
      create_option_from_v("Diffusers"),
      create_option_from_v("Core ML"),
    ]
  }
}

function create_option_from_v(val: string) {
  return {name: val, value: val}
}

function generate_boolean_type(value: boolean = false) {
  return {
    selected: { name: "", value: value },
    options: []
  }
}
