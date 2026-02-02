<script lang="ts" module>    
  
  import {Star, Image, Video, ArrowDownToLine } from "@lucide/svelte"
  
  const data = {
    downloads: [
      {
        title: "downloads",
        url: "/civit/downloads",
        icon: ArrowDownToLine ,
      },
    ],
    explore: [
      /*{
        title: "Playground",
        url: "#",
        icon: SquareTerminalIcon,
        isActive: false,
        items: [
          {
            title: "History",
            url: "#",
          },
          {
            title: "Starred",
            url: "#",
          },
          {
            title: "Settings",
            url: "#",
          },
        ],
        },*/
      {
        title: "Popular",
        url: "/civit/",
        icon: Star,
      },
      {
        title: "Images",
        url: "/civit/images",
        icon: Image,
      },
      {
        title: "Videos",
        url: "/civit/video",
        icon: Video,
      },
    ],
  };
</script>
<script lang="ts">
    import ItemMenu from "./nav-main.svelte";
    import NavUser from "./nav-user.svelte";
    import * as Sidebar from "$lib/components/ui/sidebar/index.js";
    import type { ComponentProps } from "svelte";
    let {
    ref = $bindable(null),
    collapsible = "icon",
    ...restProps
    }: ComponentProps<typeof Sidebar.Root> = $props();
</script>
<Sidebar.Root {collapsible} {...restProps}>
  <Sidebar.Content>
    <ItemMenu items={data.explore} label="Explore" />
    <ItemMenu items={data.downloads} label="" />
  </Sidebar.Content>
  <Sidebar.Footer>
    <NavUser />
  </Sidebar.Footer>
  <Sidebar.Rail />
</Sidebar.Root>