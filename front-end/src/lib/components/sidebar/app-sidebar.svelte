<script lang="ts" module>    
  import BookOpenIcon from "@lucide/svelte/icons/book-open";
  import BotIcon from "@lucide/svelte/icons/bot";
  import Settings2Icon from "@lucide/svelte/icons/settings-2";
  import SquareTerminalIcon from "@lucide/svelte/icons/square-terminal";
  
  
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
        icon: BotIcon,
      },
      {
        title: "Images",
        url: "/civit/images",
        icon: BotIcon,
      },
      {
        title: "Videos",
        url: "/civit/video",
        icon: BotIcon,
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