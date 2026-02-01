<script lang="ts" module>    
  
  import {Star, Image, Video} from "@lucide/svelte"
  
  const data = {
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
    
    
    import NavMain from "./nav-main.svelte";
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
    <NavMain items={data.explore} />
  </Sidebar.Content>
  <Sidebar.Footer>
    <NavUser />
  </Sidebar.Footer>
  <Sidebar.Rail />
</Sidebar.Root>