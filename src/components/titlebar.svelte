<script lang="ts">
    // Import necessary functions from Svelte and Tauri API
    import { onMount, onDestroy } from 'svelte';                // Svelte lifecycle functions
    import { getCurrentWindow } from '@tauri-apps/api/window';  // Tauri API to interact with the current window
    import { platform } from '@tauri-apps/plugin-os';           // Tauri API to get the platform information

    // HTML button elements to control window actions
    let minimizeElm: HTMLButtonElement; // Button to minimize the window
    let maximizeElm: HTMLButtonElement; // Button to maximize or restore the window
    let closeElm: HTMLButtonElement;    // Button to close the window

    let minimizeMacosElm: HTMLButtonElement; // Button to minimize the window on macOS
    let maximizeMacosElm: HTMLButtonElement; // Button to maximize or restore the window on macOS
    let closeMacosElm: HTMLButtonElement;    // Button to close the window on macOS

    // Elements for maximize/restore icons
    let maximizeIcon: HTMLElement; // Icon to show when window is not maximized
    let restoreIcon: HTMLElement;  // Icon to show when window is maximized

    const dropDowns: Record<string, HTMLDivElement | null> = {
        "file": null,
        "edit": null,
        "terminal": null,
        "help": null
    };

    const appWindow = getCurrentWindow();               // Get the current window instance using Tauri's API
    const minimize = () => appWindow.minimize();        // Function to minimize the window
    const maximize = () => appWindow.toggleMaximize();  // Function to toggle between maximizing and restoring the window
    const close = () => appWindow.close();              // Function to close the window

    // Function to toggle visibility of maximize/restore icons based on window state
    const toggleMaximizeIcons = () => {
        appWindow.isMaximized().then(isMaximized => {
            if (isMaximized) {
                // If the window is maximized, show the restore icon and hide the maximize icon
                maximizeIcon.classList.add("hidden");
                restoreIcon.classList.remove("hidden");
            } else {
                // If the window is not maximized, show the maximize icon and hide the restore icon
                maximizeIcon.classList.remove("hidden");
                restoreIcon.classList.add("hidden");
            }
        });
    };

    // Set up an event listener to trigger icon toggle when window is resized
    appWindow.onResized(toggleMaximizeIcons);

    // Svelte lifecycle function to run when the component is mounted to the DOM
    onMount(async () => {
        const currentPlatform = await platform();

        // Attach event listeners to buttons when the component is mounted
        if (currentPlatform === 'macos') {
            minimizeMacosElm?.addEventListener("click", minimize); // Minimize window on button click
            maximizeMacosElm?.addEventListener("click", maximize); // Maximize/restore window on button click
            closeMacosElm?.addEventListener("click", close);       // Close window on button click
        } else {
            minimizeElm?.addEventListener("click", minimize);
            maximizeElm?.addEventListener("click", maximize);
            closeElm?.addEventListener("click", close);
        }

        toggleMaximizeIcons();  // Check window state and update icons initially
    });

    // Svelte lifecycle function to run when the component is destroyed/removed from DOM
    onDestroy(async () => {
        const currentPlatform = await platform();

        // Remove event listeners from buttons to avoid memory leaks
        if (currentPlatform === 'macos') {
            minimizeMacosElm?.removeEventListener("click", minimize); // Minimize window on button click
            maximizeMacosElm?.removeEventListener("click", maximize); // Maximize/restore window on button click
            closeMacosElm?.removeEventListener("click", close);       // Close window on button click
        } else {
            minimizeElm?.removeEventListener("click", minimize);
            maximizeElm?.removeEventListener("click", maximize);
            closeElm?.removeEventListener("click", close);
        }
    });
</script>

<div data-tauri-drag-region class="h-9 max-h-9 min-h-9 pl-1 border-zinc-700 border-b flex">

    <div class="flex gap-2 place-items-center os:macos">
        <button bind:this={closeMacosElm} class="bg-[#fc5753] border-[#df4744] border-[1px] w-[13px] h-[13px] rounded-full" />
        <button bind:this={minimizeMacosElm} class="bg-[#fdbc40] border-[#de9f33] border-[1px] w-[13px] h-[13px] rounded-full" />
        <button bind:this={maximizeMacosElm} class="bg-[#36c84b] border-[#27aa35] border-[1px] w-[13px] h-[13px] rounded-full" />
    </div>

    <div class="os:windows flex gap-1 text-white place-items-center text-sm">
        <div aria-hidden="true" on:mouseleave={() => {
            dropDowns.file?.classList.add("hidden")
        }} class="relative">
            <button on:click={() => {

                for (const key in dropDowns) {
                    if (key !== "file" && dropDowns) {
                        dropDowns[key]?.classList.add("hidden")
                    }
                }

                dropDowns.file?.classList.toggle("hidden")
            }} 

            class="cursor-pointer hover:bg-white/[.1] hover:text-white text-white/[.7] px-1.5 py-0.5 rounded-md">
                File
            </button>
            <div bind:this={dropDowns.file} class="hidden text-white/[.8] text-xs flex flex-col absolute bg-window py-1.5 rounded-md top-6 z-50 border-zinc-400/[.4] border w-56">
                <span class="hover:bg-primary/[.5] cursor-pointer pl-3 py-1 rounded-md mx-1">New File</span>
                <span class="hover:bg-primary/[.5] cursor-pointer pl-3 py-1 rounded-md mx-1">New Window</span>
                <div class="border-b border-zinc-400/[.4] my-2"></div>
                <span class="hover:bg-primary/[.5] cursor-pointer pl-3 py-1 rounded-md mx-1">Open File</span>
                <span class="hover:bg-primary/[.5] cursor-pointer pl-3 py-1 rounded-md mx-1">Open Folder</span>
                <span class="hover:bg-primary/[.5] cursor-pointer pl-3 py-1 rounded-md mx-1">Open Recent</span>
                <div class="border-b border-zinc-400/[.4] my-2"></div>
                <span class="hover:bg-primary/[.5] cursor-pointer pl-3 py-1 rounded-md mx-1">Save</span>
                <span class="hover:bg-primary/[.5] cursor-pointer pl-3 py-1 rounded-md mx-1">Save As</span>
                <span class="hover:bg-primary/[.5] cursor-pointer pl-3 py-1 rounded-md mx-1">Save All</span>
                <div class="border-b border-zinc-400/[.4] my-2"></div>
                <span class="hover:bg-primary/[.5] cursor-pointer pl-3 py-1 rounded-md mx-1">Exit</span>
            </div>
        </div>

        <div aria-hidden="true" on:mouseleave={() => {
            dropDowns.edit?.classList.add("hidden")
        }} class="relative">
            <button on:click={() => {

                for (const key in dropDowns) {
                    if (key !== "edit" && dropDowns) {
                        dropDowns[key]?.classList.add("hidden")
                    }
                }

                dropDowns.edit?.classList.toggle("hidden")
            }} class="cursor-pointer hover:bg-white/[.1] hover:text-white text-white/[.7] px-1.5 py-0.5 rounded-md">
                Edit
            </button>
            <div bind:this={dropDowns.edit} class="hidden text-white/[.8] text-xs flex flex-col absolute bg-window py-1.5 rounded-md top-6 z-50 border-zinc-400/[.4] border w-56">
                <span class="hover:bg-primary/[.5] cursor-pointer pl-3 py-1 rounded-md mx-1">Undo</span>
                <span class="hover:bg-primary/[.5] cursor-pointer pl-3 py-1 rounded-md mx-1">Redo</span>
                <div class="border-b border-zinc-400/[.4] my-2"></div>
                <span class="hover:bg-primary/[.5] cursor-pointer pl-3 py-1 rounded-md mx-1">Cut</span>
                <span class="hover:bg-primary/[.5] cursor-pointer pl-3 py-1 rounded-md mx-1">Copy</span>
                <span class="hover:bg-primary/[.5] cursor-pointer pl-3 py-1 rounded-md mx-1">Paste</span>
                <div class="border-b border-zinc-400/[.4] my-2"></div>
                <span class="hover:bg-primary/[.5] cursor-pointer pl-3 py-1 rounded-md mx-1">Find</span>
                <span class="hover:bg-primary/[.5] cursor-pointer pl-3 py-1 rounded-md mx-1">Replace</span>
                <span class="hover:bg-primary/[.5] cursor-pointer pl-3 py-1 rounded-md mx-1">Find In Files</span>
                <span class="hover:bg-primary/[.5] cursor-pointer pl-3 py-1 rounded-md mx-1">Replace In Files</span>
            </div>
        </div>

        <div aria-hidden="true" on:mouseleave={() => {
            dropDowns.terminal?.classList.add("hidden")
        }} class="relative">
            <button on:click={() => {

                for (const key in dropDowns) {
                    if (key !== "terminal" && dropDowns) {
                        dropDowns[key]?.classList.add("hidden")
                    }
                }

                dropDowns.terminal?.classList.toggle("hidden")
            }} class="cursor-pointer hover:bg-white/[.1] hover:text-white text-white/[.7] px-1.5 py-0.5 rounded-md">
                Terminal
            </button>
            <div bind:this={dropDowns.terminal} class="hidden text-white/[.8] text-xs flex flex-col absolute bg-window py-1.5 rounded-md top-6 z-50 border-zinc-400/[.4] border w-56">
                <span class="hover:bg-primary/[.5] cursor-pointer pl-3 py-1 rounded-md mx-1">New Terminal</span>
                <span class="hover:bg-primary/[.5] cursor-pointer pl-3 py-1 rounded-md mx-1">Split Terminal</span>
            </div>
        </div>

        <div aria-hidden="true" on:mouseleave={() => {
            dropDowns.help?.classList.add("hidden")
        }} class="relative">
            <button on:click={() => {
                
                for (const key in dropDowns) {
                    if (key !== "help" && dropDowns) {
                        dropDowns[key]?.classList.add("hidden")
                    }
                }

                dropDowns.help?.classList.toggle("hidden")
            }} class="cursor-pointer hover:bg-white/[.1] hover:text-white text-white/[.7] px-1.5 py-0.5 rounded-md">
                Help
            </button>
            <div bind:this={dropDowns.help} class="hidden text-white/[.8] text-xs flex flex-col absolute bg-window py-1.5 rounded-md top-6 z-50 border-zinc-400/[.4] border w-56">
                <span class="hover:bg-primary/[.5] cursor-pointer pl-3 py-1 rounded-md mx-1">Welcome</span>
                <span class="hover:bg-primary/[.5] cursor-pointer pl-3 py-1 rounded-md mx-1">Github</span>
                <span class="hover:bg-primary/[.5] cursor-pointer pl-3 py-1 rounded-md mx-1">Documentation</span>
                <div class="border-b border-zinc-400/[.4] my-2"></div>
                <span class="hover:bg-primary/[.5] cursor-pointer pl-3 py-1 rounded-md mx-1">Release Notes</span>
                <span class="hover:bg-primary/[.5] cursor-pointer pl-3 py-1 rounded-md mx-1">Report Issue</span>
                <div class="border-b border-zinc-400/[.4] my-2"></div>
                <span class="hover:bg-primary/[.5] cursor-pointer pl-3 py-1 rounded-md mx-1">Restart</span>
            </div>
        </div>
    
    </div>

    <div class="grow select-none -z-50"/>

    <div class="flex gap-1 os:windows place-items-center">
        <button bind:this={minimizeElm} class="text-white py-2.5 px-3 hover:bg-white/[.1]">
            <svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 16 16"><path fill="currentColor" d="M14 8v1H3V8z"/></svg>
        </button>
        <button bind:this={maximizeElm} class="text-white py-2.5 px-3 hover:bg-white/[.1]">
            <span bind:this={maximizeIcon}><svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 16 16"><path fill="currentColor" d="M3 3v10h10V3zm9 9H4V4h8z"/></svg></span>
            <span bind:this={restoreIcon} class="hidden"><svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 16 16"><g fill="currentColor"><path d="M3 5v9h9V5zm8 8H4V6h7z"/><path fill-rule="evenodd" d="M5 5h1V4h7v7h-1v1h2V3H5z" clip-rule="evenodd"/></g></svg></span>
        </button>
        <button bind:this={closeElm} class="text-white py-2.5 px-3 hover:bg-red-400/[.1]">
            <svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 16 16"><path fill="currentColor" fill-rule="evenodd" d="m7.116 8l-4.558 4.558l.884.884L8 8.884l4.558 4.558l.884-.884L8.884 8l4.558-4.558l-.884-.884L8 7.116L3.442 2.558l-.884.884z" clip-rule="evenodd"/></svg>
        </button>
    </div>

</div>