<script lang="ts">
    import Sidebar from "$lib/Sidebar.svelte";
    import ReleaseArt from "$lib/ReleaseArt.svelte";
    import { Separator } from "$lib/ui/ui/separator/index.js";
    import { ScrollArea } from "$lib/ui/ui/scroll-area/index.js";
    import { Input } from "$lib/ui/ui/input/index.js";
    import type { Release } from "$lib/types";
    import SearchResults from "$lib/SearchResults.svelte";
    import AddMusic from "$lib/AddMusic.svelte";
    import type { PageData } from "./$types";

    let playlists: string[] = ["Test"];
    let releases: Release[] = [
        {
            id: 1,
            name: "Test",
            performers: [
                {
                    id: 1,
                    name: "Test Artist",
                    description: null,
                    artistType: "performer",
                    imagePath: null,
                },
            ],
            description: null,
            imagePath: "https://via.placeholder.com/150",
        },
    ];

    let searchQuery = "";

    // get token from page data
    export let data: PageData;
    const { token } = data;
</script>

<div class="hidden md:block dark">
    <div class="border-t border-slate-800">
        <div class="bg-slate-950 text-slate-50">
            <div class="grid lg:grid-cols-5">
                <Sidebar {playlists} class="hidden lg:block" />
                <div
                    class="col-span-3 lg:col-span-4 lg:border-l lg:border-slate-800"
                >
                    <div class="h-full px-4 py-6 lg:px-8">
                        <div class="space-between py-3 flex items-center">
                            <div class="flex w-full max-w-sm items-center">
                                <Input
                                    type="search"
                                    placeholder="Search music..."
                                    class="mr-4 bg-slate-900 border-slate-800"
                                    bind:value={searchQuery}
                                />
                            </div>
                            <div class="ml-auto mr-4">
                                <AddMusic />
                            </div>
                        </div>

                        {#if searchQuery}
                            <SearchResults {searchQuery} {token} />
                        {:else}
                            <div class="flex items-center justify-between">
                                <div class="space-y-1">
                                    <h2
                                        class="text-2xl font-semibold tracking-tight text-slate-50"
                                    >
                                        Listen Now
                                    </h2>
                                    <p class="text-slate-400 text-sm">
                                        Top picks for you. Updated daily.
                                    </p>
                                </div>
                            </div>
                            <Separator class="my-4 bg-slate-800" />
                            <div class="relative">
                                <ScrollArea orientation="both">
                                    <div class="flex space-x-4 pb-4">
                                        {#each releases as release}
                                            <ReleaseArt
                                                {release}
                                                class="w-[250px]"
                                                aspectRatio="portrait"
                                                width={250}
                                                height={330}
                                            />
                                        {/each}
                                    </div>
                                </ScrollArea>
                            </div>
                            <div class="mt-6 space-y-1">
                                <h2
                                    class="text-2xl font-semibold tracking-tight text-slate-50"
                                >
                                    Made for You
                                </h2>
                                <p class="text-slate-400 text-sm">
                                    Your personal playlists. Updated daily.
                                </p>
                            </div>
                            <Separator class="my-4 bg-slate-800" />
                            <div class="relative">
                                <ScrollArea orientation="both">
                                    <div class="flex space-x-4 pb-4">
                                        {#each releases as release}
                                            <ReleaseArt
                                                {release}
                                                class="w-[150px]"
                                                aspectRatio="square"
                                                width={150}
                                                height={150}
                                            />
                                        {/each}
                                    </div>
                                </ScrollArea>
                            </div>
                        {/if}
                    </div>
                </div>
            </div>
        </div>
    </div>
</div>
