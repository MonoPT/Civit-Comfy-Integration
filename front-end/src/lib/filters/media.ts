export let media_filters = {
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
    selected: {name: "Most Reactions", value: "Most Reactions"},
    options: [
      {name: "Most Reactions", value: "Most Reactions"},
      {name: "Most Comments", value: "Most Comments"},
      {name: "Most Collected", value: "Most Collected"},
      {name: "Newest", value: "Newest"},
      {name: "Oldest", value: "Oldest"}
    ]
  },
  browsingLevel: { // NSFW Filter
    selected: { name: "Hide Nsfw", value: "0" },
    options: [
      { name: "Hide Nsfw", value: "0" },
      { name: "Include Nsfw", value: "31" },
      { name: "Only Nsfw", value: "100" },
    ]
  },
  baseModel: {
    selected: { name: "", value: "" },
    options: []
  }
}