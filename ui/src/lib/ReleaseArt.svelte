<script lang="ts">
    import type { Release } from "$lib/types";
    import { cn } from "$lib/utils";
    import * as ContextMenu from "$lib/ui/ui/context-menu/index.js";
    import { goto } from "$app/navigation";

    let className: string | undefined | null = undefined;
    export let release: Release;
    export let aspectRatio: "portrait" | "square" = "square";
    export let width: number;
    export let height: number;
    export { className as class };

    // navigate to release page
    function handleClick() {
        goto(`/allegro/release/${release.id}`);
    }

    // get performer names as string
    $: artistNames = release.performers.map((p) => p.name).join(", ");
</script>

<div
    class={cn("space-y-3", className)}
    on:click={handleClick}
    on:keydown={(e) => e.key === "Enter" && handleClick()}
    role="button"
    tabindex="0"
    {...$$restProps}
>
    <ContextMenu.Root>
        <ContextMenu.Trigger>
            <div class="overflow-hidden rounded-md bg-slate-900">
                <img
                    class={cn(
                        "h-auto w-auto object-cover transition-all hover:scale-105",
                        `w-[${width}px]`,
                        `h-[${height}px]`,
                        aspectRatio === "portrait"
                            ? "aspect-[3/4]"
                            : "aspect-square",
                    )}
                    src={"/uploads/" + release.imagePath}
                    alt={release.name}
                />
            </div>
        </ContextMenu.Trigger>
        <ContextMenu.Content class="w-40 bg-slate-900 border-slate-800">
            <ContextMenu.Item
                class="text-slate-200 hover:bg-slate-800 focus:bg-slate-800"
                >Add to Library</ContextMenu.Item
            >
            <ContextMenu.Separator class="bg-slate-800" />
            <ContextMenu.Item
                class="text-slate-200 hover:bg-slate-800 focus:bg-slate-800"
                >Play Next</ContextMenu.Item
            >
            <ContextMenu.Item
                class="text-slate-200 hover:bg-slate-800 focus:bg-slate-800"
                >Play Later</ContextMenu.Item
            >
            <ContextMenu.Item
                class="text-slate-200 hover:bg-slate-800 focus:bg-slate-800"
                >Create Station</ContextMenu.Item
            >
            <ContextMenu.Separator class="bg-slate-800" />
            <ContextMenu.Item
                class="text-slate-200 hover:bg-slate-800 focus:bg-slate-800"
                >Like</ContextMenu.Item
            >
            <ContextMenu.Item
                class="text-slate-200 hover:bg-slate-800 focus:bg-slate-800"
                >Share</ContextMenu.Item
            >
        </ContextMenu.Content>
    </ContextMenu.Root>
    <div class="space-y-1 text-sm">
        <h3 class="font-medium leading-none text-slate-200">{release.name}</h3>
        <p class="text-slate-400 text-xs">{artistNames}</p>
    </div>
</div>
