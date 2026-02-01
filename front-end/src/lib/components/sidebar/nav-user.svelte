<script lang="ts">
    import {userState, logoutUser} from "$lib/state.svelte"
  
    import ChevronsUpDownIcon from "@lucide/svelte/icons/chevrons-up-down";
    import LogOutIcon from "@lucide/svelte/icons/log-out";
    
    import * as Avatar from "$lib/components/ui/avatar/index.js";
    import * as DropdownMenu from "$lib/components/ui/dropdown-menu/index.js";
    import * as Sidebar from "$lib/components/ui/sidebar/index.js";
    import { useSidebar } from "$lib/components/ui/sidebar/index.js";
    import { Button } from "$lib/components/ui/button/index.js";
    
    import {SunIcon, MoonIcon} from "@lucide/svelte"
    
    import { toggleMode } from "mode-watcher";

    const sidebar = useSidebar();
    
</script>
<Sidebar.Menu>
  <Sidebar.MenuItem>
    <DropdownMenu.Root>
      <DropdownMenu.Trigger>
        {#snippet child({ props })}
          <Sidebar.MenuButton
            {...props}
            size="lg"
            class="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
          >
            <Avatar.Root class="size-8 rounded-lg">
              <Avatar.Image src={userState.image} alt={userState.user_name} />
              <Avatar.Fallback class="rounded-lg">CN</Avatar.Fallback>
            </Avatar.Root>
            <div class="grid flex-1 text-start text-sm leading-tight">
              <span class="truncate font-medium">{userState.user_name}</span>
              <span class="truncate text-xs">{userState.user_email}</span>
            </div>
            <ChevronsUpDownIcon class="ms-auto size-4" />
          </Sidebar.MenuButton>
        {/snippet}
      </DropdownMenu.Trigger>
      <DropdownMenu.Content
        class="w-(--bits-dropdown-menu-anchor-width) min-w-56 rounded-lg"
        side={sidebar.isMobile ? "bottom" : "right"}
        align="end"
        sideOffset={4}
      >
        <DropdownMenu.Label class="p-0 font-normal">
          <div class="flex items-center gap-2 px-1 py-1.5 text-start text-sm">
            <Avatar.Root class="size-8 rounded-lg">
              <Avatar.Image src={userState.image} alt={userState.user_name} />
              <Avatar.Fallback class="rounded-lg">CN</Avatar.Fallback>
            </Avatar.Root>
            <div class="grid flex-1 text-start text-sm leading-tight">
              <span class="truncate font-medium">{userState.user_name}</span>
              <span class="truncate text-xs">{userState.user_email}</span>
            </div>
          </div>
        </DropdownMenu.Label>
        <DropdownMenu.Separator />
        
        <span onclick={toggleMode}>
            <DropdownMenu.Item> 
                <SunIcon 
                    class="h-[1.2rem] w-[1.2rem] scale-100 rotate-0 transition-all! dark:scale-0 dark:-rotate-90"
                />
                <MoonIcon
                    class="absolute h-[1.2rem] w-[1.2rem] scale-0 rotate-90 transition-all! dark:scale-100 dark:rotate-0"
                />
               <span>Toggle theme</span>
            </DropdownMenu.Item>
        </span>
        
        <DropdownMenu.Separator />
        <span onclick={logoutUser}>
            <DropdownMenu.Item> 
              <LogOutIcon />
              Log out
            </DropdownMenu.Item>
        </span>
      </DropdownMenu.Content>
    </DropdownMenu.Root>
  </Sidebar.MenuItem>
</Sidebar.Menu>