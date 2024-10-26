<script lang="ts">
    import { page } from "$app/stores";
    import axios from "axios";
    import type {
        DbRecording,
        DbRelease,
        Recording,
        Release,
        TokenData,
    } from "$lib/types";
    import { Button } from "$lib/ui/ui/button";
    import { ArrowLeft } from "lucide-svelte";
    import { onMount } from "svelte";
    import { goto } from "$app/navigation";
    import Sidebar from "$lib/Sidebar.svelte";
    import { Input } from "$lib/ui/ui/input";
    import { Separator } from "$lib/ui/ui/separator";
    import SearchResults from "$lib/SearchResults.svelte";
    import AddMusic from "$lib/AddMusic.svelte";
    import { buildRecording, buildRelease } from "$lib/music";
    import TracksTable from "$lib/TracksTable.svelte";

    const releaseId = $page.params.id;
    let release: Release | null = null;
    let recordings: Recording[] = [];
    let searchQuery = "";
    let playlists: string[] = ["Test"];

    interface RecordingResponse {
        success: boolean;
        message: DbRecording[];
    }

    interface ReleaseResponse {
        success: boolean;
        message: DbRelease;
    }

    // get release data
    onMount(async () => {
        try {
            const releaseResponse = await axios.post<ReleaseResponse>(
                "http://localhost:9000/music/get/release",
                {
                    id: parseInt(releaseId),
                },
            );
            release = await buildRelease(releaseResponse.data.message);

            // get recordings for this release
            const recordingsResponse = await axios.post<RecordingResponse>(
                "http://localhost:9000/music/get/recordings",
                {
                    id: parseInt(releaseId),
                },
            );
            console.log(recordingsResponse.data);

            let db_recordings: DbRecording[] = [];
            if (recordingsResponse.data.message) {
                // sort recordings by track number
                db_recordings = recordingsResponse.data.message;
                db_recordings.sort((a, b) => a.track_number - b.track_number);

                // build Recording objects
                let tempRecordings = [];
                for (let db_recording of db_recordings) {
                    let recording = await buildRecording(db_recording);
                    tempRecordings.push(recording);
                }
                recordings = tempRecordings;
            }

            console.log(recordings);
        } catch (error) {
            console.error("Error fetching release data:", error);
        }
    });

    function goBack() {
        goto("/allegro");
    }

    // get token from page data
    export let data: TokenData;
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
                            <!-- Release Content -->
                            <div class="space-y-1">
                                <Button
                                    variant="ghost"
                                    class="mb-2 text-slate-400 hover:text-slate-50 -ml-2"
                                    on:click={goBack}
                                >
                                    <ArrowLeft class="mr-2 h-4 w-4" />
                                    Back
                                </Button>
                            </div>
                            <Separator class="my-4 bg-slate-800" />

                            {#if release}
                                <div
                                    class="grid md:grid-cols-[300px_1fr] gap-8"
                                >
                                    <!-- Release Cover and Info -->
                                    <div>
                                        <div
                                            class="rounded-lg overflow-hidden mb-4"
                                        >
                                            <img
                                                src={release.imagePath
                                                    ? `/uploads/${release.imagePath}`
                                                    : "https://via.placeholder.com/300"}
                                                alt={release.name}
                                                class="w-full h-auto"
                                            />
                                        </div>
                                        <h1 class="text-2xl font-bold mb-2">
                                            {release.name}
                                        </h1>
                                        <div class="text-slate-400 mb-4">
                                            {#if release.performers}
                                                {release.performers
                                                    .map((p) => p.name)
                                                    .join(", ")}
                                            {/if}
                                        </div>
                                        {#if release.description}
                                            <p class="text-slate-300">
                                                {release.description}
                                            </p>
                                        {/if}
                                    </div>

                                    <!-- Tracklist -->
                                    <div>
                                        <h2 class="text-xl font-semibold mb-4">
                                            Tracklist
                                        </h2>
                                        <TracksTable {recordings} />
                                    </div>
                                </div>
                            {:else}
                                <div
                                    class="flex justify-center items-center h-[50vh]"
                                >
                                    <div class="text-slate-400">Loading...</div>
                                </div>
                            {/if}
                        {/if}
                    </div>
                </div>
            </div>
        </div>
    </div>
</div>
