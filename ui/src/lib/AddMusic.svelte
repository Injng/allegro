<script lang="ts">
    import * as Alert from "$lib/ui/ui/alert";
    import { Button } from "$lib/ui/ui/button";
    import * as Command from "$lib/ui/ui/command";
    import { Input } from "$lib/ui/ui/input";
    import { Label } from "$lib/ui/ui/label";
    import * as Tabs from "$lib/ui/ui/tabs";
    import * as Dialog from "$lib/ui/ui/dialog";
    import TextArea from "$lib/TextArea.svelte";

    import { enhance } from "$app/forms";
    import type { ActionResult } from "@sveltejs/kit";
    import { fade } from "svelte/transition";
    import type { Artist, Piece, Release } from "./types";
    import axios from "axios";

    // ensure files are less than 300 MB
    function handleFileChange(event: Event) {
        const target = event.target as HTMLInputElement;
        if (target.files?.[0]) {
            if (target.files[0].size > 300 * 1024 * 1024) {
                alert("File size must be less than 300 MB");
                target.value = "";
                return;
            }
        }
    }

    // handle submission by displaying alert for success or error
    let showAlert = false;
    let alertType: "success" | "error" = "success";
    let alertMessage = "";
    function handleSubmit() {
        return async ({ result }: { result: ActionResult }) => {
            if (result.type === "success") {
                alertType = "success";
                alertMessage = "Successfully added";
                showAlert = true;
            } else if (result.type === "failure") {
                alertType = "error";
                alertMessage = `Error: ${result.data?.error || "An unknown error occurred"}`;
                showAlert = true;
            } else if (result.type === "redirect") {
                alertType = "error";
                alertMessage = "Error: Server API error";
                showAlert = true;
            }

            // hide alert after a few seconds
            setTimeout(() => {
                showAlert = false;
            }, 3000);
        };
    }

    // open search dialog
    let composerDialogOpen = false;
    let songwriterDialogOpen = false;
    let performerDialogOpen = false;
    let pieceDialogOpen = false;
    let releaseDialogOpen = false;

    // handle selected artists
    let selectedPerformers: Artist[] = [];
    let selectedComposers: Artist[] = [];
    let selectedSongwriters: Artist[] = [];

    // handle search dialog select
    function handleArtistSelect(
        artist: Artist,
        artist_type: "performer" | "composer" | "songwriter",
    ) {
        if (artist_type === "performer") {
            // check if already selected
            const index = selectedPerformers.findIndex(
                (p) => p.id === artist.id,
            );
            if (index === -1) {
                selectedPerformers = [...selectedPerformers, artist];
            } else {
                selectedPerformers = selectedPerformers.filter(
                    (p) => p.id !== artist.id,
                );
            }
        } else if (artist_type === "composer") {
            // check if already selected
            const index = selectedComposers.findIndex(
                (c) => c.id === artist.id,
            );
            if (index === -1) {
                selectedComposers = [...selectedComposers, artist];
            } else {
                selectedComposers = selectedComposers.filter(
                    (c) => c.id !== artist.id,
                );
            }
        } else if (artist_type === "songwriter") {
            // check if already selected
            const index = selectedSongwriters.findIndex(
                (s) => s.id === artist.id,
            );
            if (index === -1) {
                selectedSongwriters = [...selectedSongwriters, artist];
            } else {
                selectedSongwriters = selectedSongwriters.filter(
                    (s) => s.id !== artist.id,
                );
            }
        }
    }

    // handle piece select
    let selectedPieceName = "";
    let selectedPieceId: number = 0;
    function handlePieceSelect(piece: Piece) {
        selectedPieceName = piece.name;
        selectedPieceId = piece.id;
        pieceDialogOpen = false;
    }

    // handle release select
    let selectedReleaseName = "";
    let selectedReleaseId: number = 0;
    function handleReleaseSelect(release: Release) {
        selectedReleaseName = release.name;
        selectedReleaseId = release.id;
        releaseDialogOpen = false;
    }

    // set axios api
    const api = axios.create({
        baseURL: "http://localhost:9000",
    });

    // search for pieces
    let pieceSearch = "";
    let loadingPieces = true;
    let pieces: Piece[] = [];
    async function getPieces() {
        pieces = [];
        loadingPieces = true;
        pieceDialogOpen = true;
        try {
            const response = await api.get("/music/get/pieces");
            for (let p of response.data.message) {
                // get the composers
                let composers: Artist[] = [];
                for (let composer_id of p.composer_ids) {
                    const composerResponse = await api.post(
                        "/music/get/composer",
                        {
                            id: composer_id,
                        },
                    );
                    const composerData = composerResponse.data.message;
                    let composer: Artist = {
                        id: composerData.id,
                        name: composerData.name,
                        description: composerData.description,
                        artist_type: "composer",
                        image_path: composerData.image_path,
                    };
                    composers.push(composer);
                }

                // get the songwriters
                let songwriters: Artist[] = [];
                for (let songwriter_id of p.songwriter_ids) {
                    const songwriterResponse = await api.post(
                        "/music/get/songwriter",
                        {
                            id: songwriter_id,
                        },
                    );
                    const songwriterData = songwriterResponse.data.message;
                    let songwriter: Artist = {
                        id: songwriterData.id,
                        name: songwriterData.name,
                        description: songwriterData.description,
                        artist_type: "songwriter",
                        image_path: songwriterData.image_path,
                    };
                    songwriters.push(songwriter);
                }

                // construct the piece
                let piece: Piece = {
                    id: p.id,
                    name: p.name,
                    movements: p.movements,
                    composers,
                    songwriters,
                    description: p.description,
                };
                pieces.push(piece);
            }
        } catch (error) {
            console.error("Error fetching pieces:", error);
            pieces = [];
        } finally {
            loadingPieces = false;
        }
    }

    // search for releases
    let releaseSearch = "";
    let loadingReleases = true;
    let releases: Release[] = [];
    async function getReleases() {
        releases = [];
        loadingReleases = true;
        releaseDialogOpen = true;
        try {
            const response = await api.get("/music/get/releases");
            for (let r of response.data.message) {
                // get the performers
                let performers: Artist[] = [];
                for (let performer_id of r.performer_ids) {
                    const performerResponse = await api.post(
                        "/music/get/performer",
                        {
                            id: performer_id,
                        },
                    );
                    const data = performerResponse.data.message;
                    let performer: Artist = {
                        id: data.id,
                        name: data.name,
                        description: data.description,
                        artist_type: "performer",
                        image_path: data.image_path,
                    };
                    performers.push(performer);
                }

                // construct the release
                let release: Release = {
                    id: r.id,
                    name: r.name,
                    performers,
                    description: r.description,
                    image_path: r.image_path,
                };
                releases.push(release);
            }
        } catch (error) {
            console.error("Error fetching releases:", error);
            releases = [];
        } finally {
            loadingReleases = false;
        }
    }

    // search for artists
    let artistSearch = "";
    let loadingArtists = true;
    let artists: Artist[] = [];
    async function getArtists(
        artist_type: "performer" | "composer" | "songwriter",
    ) {
        // set the correct dialog state
        if (artist_type === "composer") {
            composerDialogOpen = true;
        } else if (artist_type === "songwriter") {
            songwriterDialogOpen = true;
        } else if (artist_type === "performer") {
            performerDialogOpen = true;
        }

        // try loading the artists
        artists = [];
        loadingArtists = true;
        try {
            const response = await api.get(`/music/get/${artist_type}s`);
            for (let a of response.data.message) {
                let artist: Artist = {
                    id: a.id,
                    name: a.name,
                    description: a.description,
                    artist_type: artist_type,
                    image_path: a.image_path,
                };
                artists.push(artist);
            }
        } catch (error) {
            console.error("Error fetching artists:", error);
            artists = [];
        } finally {
            loadingArtists = false;
        }
    }
</script>

<Dialog.Root>
    <Dialog.Trigger>
        <Button variant="secondary" class="bg-slate-800 hover:bg-slate-700">
            Add music
        </Button>
    </Dialog.Trigger>

    <Dialog.Content class="sm:max-w-[425px] bg-slate-900 text-slate-50">
        <Dialog.Header>
            <Dialog.Title class="text-slate-50">Add music</Dialog.Title>
            <Dialog.Description class="text-slate-400">
                Upload recordings and set metadata here.
            </Dialog.Description>
        </Dialog.Header>

        <div class="fixed bottom-4 left-4 z-100">
            {#if showAlert}
                <div out:fade={{ duration: 300 }}>
                    <Alert.Root
                        variant={alertType === "success"
                            ? "default"
                            : "destructive"}
                        class="mb-2 max-w-md"
                    >
                        <Alert.Title
                            >{alertType === "success"
                                ? "Success"
                                : "Error"}</Alert.Title
                        >
                        <Alert.Description>{alertMessage}</Alert.Description>
                    </Alert.Root>
                </div>
            {/if}
        </div>

        <Tabs.Root value="recording" class="w-full">
            <Tabs.List
                class="grid w-full grid-cols-4 bg-slate-800 rounded-t-lg"
            >
                <Tabs.Trigger
                    value="recording"
                    class="px-4 py-2 text-sm font-medium transition-all data-[state=active]:bg-slate-700 data-[state=active]:text-slate-50 text-slate-400 hover:text-slate-50"
                >
                    Recording
                </Tabs.Trigger>
                <Tabs.Trigger
                    value="piece"
                    class="px-4 py-2 text-sm font-medium transition-all data-[state=active]:bg-slate-700 data-[state=active]:text-slate-50 text-slate-400 hover:text-slate-50"
                >
                    Piece
                </Tabs.Trigger>
                <Tabs.Trigger
                    value="release"
                    class="px-4 py-2 text-sm font-medium transition-all data-[state=active]:bg-slate-700 data-[state=active]:text-slate-50 text-slate-400 hover:text-slate-50"
                >
                    Release
                </Tabs.Trigger>
                <Tabs.Trigger
                    value="artist"
                    class="px-4 py-2 text-sm font-medium transition-all data-[state=active]:bg-slate-700 data-[state=active]:text-slate-50 text-slate-400 hover:text-slate-50"
                >
                    Artist
                </Tabs.Trigger>
            </Tabs.List>

            <Tabs.Content value="recording" class="space-y-4">
                <form
                    method="POST"
                    enctype="multipart/form-data"
                    action="?/addrecording"
                    use:enhance={handleSubmit}
                >
                    <div class="grid gap-4 py-4">
                        <div class="grid grid-cols-4 items-center gap-4">
                            <Label for="piece" class="text-right text-slate-400"
                                >Piece*</Label
                            >
                            <div class="col-span-3">
                                <Button
                                    class="bg-slate-800 hover:bg-slate-700 text-slate-50"
                                    on:click={() => {
                                        getPieces();
                                    }}
                                >
                                    {selectedPieceName ||
                                        "Search for a piece..."}
                                </Button>
                                <input
                                    type="hidden"
                                    name="piece"
                                    value={selectedPieceId}
                                />
                                <Command.Dialog
                                    bind:open={pieceDialogOpen}
                                    class="bg-slate-900 border border-slate-700"
                                >
                                    <Command.Input
                                        placeholder="Search pieces..."
                                        class="border-none bg-slate-900 text-slate-50 placeholder:text-slate-400"
                                        bind:value={pieceSearch}
                                    />
                                    <Command.List
                                        class="bg-slate-900 text-slate-50"
                                    >
                                        {#if loadingPieces}
                                            <Command.Empty
                                                class="py-6 text-center text-sm text-slate-400"
                                            >
                                                Loading...
                                            </Command.Empty>
                                        {:else}
                                            <Command.Group class="p-1">
                                                {#each pieces as piece (piece.id)}
                                                    <Command.Item
                                                        value={piece.name}
                                                        onSelect={() => {
                                                            handlePieceSelect(
                                                                piece,
                                                            );
                                                        }}
                                                        class="cursor-pointer select-none relative text-slate-400 flex items-center rounded-sm px-2 py-1.5 text-sm outline-none aria-selected:bg-slate-700 aria-selected:text-slate-50 data-[disabled]:pointer-events-none data-[disabled]:opacity-50 hover:bg-slate-800"
                                                    >
                                                        {piece.name}
                                                    </Command.Item>
                                                {/each}
                                            </Command.Group>
                                        {/if}
                                    </Command.List>
                                </Command.Dialog>
                            </div>
                        </div>

                        <div class="grid grid-cols-4 items-center gap-4">
                            <Label
                                for="release"
                                class="text-right text-slate-400"
                                >Release*</Label
                            >
                            <div class="col-span-3">
                                <Button
                                    class="bg-slate-800 hover:bg-slate-700 text-slate-50"
                                    on:click={() => {
                                        getReleases();
                                    }}
                                >
                                    {selectedReleaseName ||
                                        "Search for a release..."}
                                </Button>
                                <input
                                    type="hidden"
                                    name="release"
                                    value={selectedReleaseId}
                                />
                                <Command.Dialog
                                    bind:open={releaseDialogOpen}
                                    class="bg-slate-900 border border-slate-700"
                                >
                                    <Command.Input
                                        placeholder="Search pieces..."
                                        class="border-none bg-slate-900 text-slate-50 placeholder:text-slate-400"
                                        bind:value={releaseSearch}
                                    />
                                    <Command.List
                                        class="bg-slate-900 text-slate-50"
                                    >
                                        {#if loadingReleases}
                                            <Command.Empty
                                                class="py-6 text-center text-sm text-slate-400"
                                            >
                                                Loading...
                                            </Command.Empty>
                                        {:else}
                                            <Command.Group class="p-1">
                                                {#each releases as release (release.id)}
                                                    <Command.Item
                                                        value={release.name}
                                                        onSelect={() => {
                                                            handleReleaseSelect(
                                                                release,
                                                            );
                                                        }}
                                                        class="cursor-pointer select-none relative text-slate-400 flex items-center rounded-sm px-2 py-1.5 text-sm outline-none aria-selected:bg-slate-700 aria-selected:text-slate-50 data-[disabled]:pointer-events-none data-[disabled]:opacity-50 hover:bg-slate-800"
                                                    >
                                                        {release.name}
                                                    </Command.Item>
                                                {/each}
                                            </Command.Group>
                                        {/if}
                                    </Command.List>
                                </Command.Dialog>
                            </div>
                        </div>

                        <div class="grid grid-cols-4 items-center gap-4">
                            <Label
                                for="performer"
                                class="text-right text-slate-400"
                                >Performer*</Label
                            >
                            <div class="col-span-3">
                                <div class="flex flex-wrap gap-2 mb-2">
                                    {#each selectedPerformers as performer}
                                        <div
                                            class="bg-slate-700 px-2 py-1 rounded-md flex items-center gap-2"
                                        >
                                            {performer.name}
                                            <button
                                                class="text-slate-400 hover:text-slate-200"
                                                on:click={() =>
                                                    handleArtistSelect(
                                                        performer,
                                                        "performer",
                                                    )}>×</button
                                            >
                                        </div>
                                    {/each}
                                </div>
                                <Button
                                    class="bg-slate-800 hover:bg-slate-700 text-slate-50"
                                    on:click={() => {
                                        getArtists("performer");
                                    }}
                                >
                                    Add performer...
                                </Button>
                                <input
                                    type="hidden"
                                    name="performers"
                                    value={selectedPerformers
                                        .map((p) => p.id)
                                        .join(",")}
                                />
                                <Command.Dialog
                                    bind:open={performerDialogOpen}
                                    class="bg-slate-900 border border-slate-700"
                                >
                                    <Command.Input
                                        placeholder="Search artists..."
                                        class="border-none bg-slate-900 text-slate-50 placeholder:text-slate-400"
                                        bind:value={artistSearch}
                                    />
                                    <Command.List
                                        class="bg-slate-900 text-slate-50"
                                    >
                                        {#if loadingArtists}
                                            <Command.Empty
                                                class="py-6 text-center text-sm text-slate-400"
                                            >
                                                Loading...
                                            </Command.Empty>
                                        {:else}
                                            <Command.Group class="p-1">
                                                {#each artists as artist (artist.id)}
                                                    <Command.Item
                                                        value={artist.name}
                                                        onSelect={() =>
                                                            handleArtistSelect(
                                                                artist,
                                                                "performer",
                                                            )}
                                                        class="cursor-pointer select-none relative text-slate-400 flex items-center rounded-sm px-2 py-1.5 text-sm outline-none aria-selected:bg-slate-700 aria-selected:text-slate-50 data-[disabled]:pointer-events-none data-[disabled]:opacity-50 hover:bg-slate-800"
                                                    >
                                                        {artist.name}
                                                    </Command.Item>
                                                {/each}
                                            </Command.Group>
                                        {/if}
                                    </Command.List>
                                </Command.Dialog>
                            </div>
                        </div>

                        <div class="grid grid-cols-4 items-center gap-4">
                            <Label for="track" class="text-right text-slate-400"
                                >Track #</Label
                            >
                            <Input
                                type="number"
                                id="track"
                                name="track"
                                class="col-span-3 bg-slate-800 border-slate-700 text-slate-50"
                                min={1}
                                step={1}
                                required
                            />
                        </div>

                        <div class="grid grid-cols-4 items-center gap-4">
                            <Label for="file" class="text-right text-slate-400"
                                >Recording*</Label
                            >
                            <Input
                                type="file"
                                id="file"
                                name="file"
                                accept="audio/*"
                                on:change={handleFileChange}
                                class="col-span-3 bg-slate-800 border-slate-700 text-slate-50 file:bg-slate-700 file:text-slate-50 file:border-0"
                                required
                            />
                        </div>
                    </div>
                    <div class="flex justify-end mt-4">
                        <Button
                            type="submit"
                            class="bg-slate-800 hover:bg-slate-700 text-slate-50"
                            >Add recording</Button
                        >
                    </div>
                </form></Tabs.Content
            >

            <Tabs.Content value="piece">
                <form
                    method="POST"
                    enctype="multipart/form-data"
                    action="?/addpiece"
                    use:enhance={handleSubmit}
                >
                    <div class="grid gap-4 py-4">
                        <div class="grid grid-cols-4 items-center gap-4">
                            <Label for="name" class="text-right text-slate-400"
                                >Name*</Label
                            >
                            <Input
                                id="name"
                                name="name"
                                class="col-span-3 bg-slate-800 border-slate-700 text-slate-50"
                                required
                            />
                        </div>

                        <div class="grid grid-cols-4 items-center gap-4">
                            <Label
                                for="movements"
                                class="text-right text-slate-400"
                                >Movements</Label
                            >
                            <Input
                                type="number"
                                id="movements"
                                name="movements"
                                class="col-span-3 bg-slate-800 border-slate-700 text-slate-50"
                                min={0}
                                step={1}
                            />
                        </div>

                        <div class="grid grid-cols-4 items-center gap-4">
                            <Label
                                for="composer"
                                class="text-right text-slate-400"
                                >Composer*</Label
                            >
                            <div class="col-span-3">
                                <div class="flex flex-wrap gap-2 mb-2">
                                    {#each selectedComposers as composer}
                                        <div
                                            class="bg-slate-700 px-2 py-1 rounded-md flex items-center gap-2"
                                        >
                                            {composer.name}
                                            <button
                                                class="text-slate-400 hover:text-slate-200"
                                                on:click={() =>
                                                    handleArtistSelect(
                                                        composer,
                                                        "composer",
                                                    )}>×</button
                                            >
                                        </div>
                                    {/each}
                                </div>
                                <Button
                                    class="bg-slate-800 hover:bg-slate-700 text-slate-50"
                                    on:click={() => {
                                        getArtists("composer");
                                    }}
                                >
                                    Add composer...
                                </Button>
                                <input
                                    type="hidden"
                                    name="composers"
                                    value={selectedComposers
                                        .map((p) => p.id)
                                        .join(",")}
                                />
                                <Command.Dialog
                                    bind:open={composerDialogOpen}
                                    class="bg-slate-900 border border-slate-700"
                                >
                                    <Command.Input
                                        placeholder="Search artists..."
                                        class="border-none bg-slate-900 text-slate-50 placeholder:text-slate-400"
                                        bind:value={artistSearch}
                                    />
                                    <Command.List
                                        class="bg-slate-900 text-slate-50"
                                    >
                                        {#if loadingArtists}
                                            <Command.Empty
                                                class="py-6 text-center text-sm text-slate-400"
                                            >
                                                Loading...
                                            </Command.Empty>
                                        {:else}
                                            <Command.Group class="p-1">
                                                {#each artists as artist (artist.id)}
                                                    <Command.Item
                                                        value={artist.name}
                                                        onSelect={() =>
                                                            handleArtistSelect(
                                                                artist,
                                                                "composer",
                                                            )}
                                                        class="cursor-pointer select-none relative text-slate-400 flex items-center rounded-sm px-2 py-1.5 text-sm outline-none aria-selected:bg-slate-700 aria-selected:text-slate-50 data-[disabled]:pointer-events-none data-[disabled]:opacity-50 hover:bg-slate-800"
                                                    >
                                                        {artist.name}
                                                    </Command.Item>
                                                {/each}
                                            </Command.Group>
                                        {/if}
                                    </Command.List>
                                </Command.Dialog>
                            </div>
                        </div>

                        <div class="grid grid-cols-4 items-center gap-4">
                            <Label
                                for="songwriter"
                                class="text-right text-slate-400"
                                >Songwriter</Label
                            >
                            <div class="col-span-3">
                                <div class="flex flex-wrap gap-2 mb-2">
                                    {#each selectedSongwriters as songwriter}
                                        <div
                                            class="bg-slate-700 px-2 py-1 rounded-md flex items-center gap-2"
                                        >
                                            {songwriter.name}
                                            <button
                                                class="text-slate-400 hover:text-slate-200"
                                                on:click={() =>
                                                    handleArtistSelect(
                                                        songwriter,
                                                        "songwriter",
                                                    )}>×</button
                                            >
                                        </div>
                                    {/each}
                                </div>
                                <Button
                                    class="bg-slate-800 hover:bg-slate-700 text-slate-50"
                                    on:click={() => {
                                        getArtists("songwriter");
                                    }}
                                >
                                    Add songwriter...
                                </Button>
                                <input
                                    type="hidden"
                                    name="songwriters"
                                    value={selectedSongwriters
                                        .map((p) => p.id)
                                        .join(",")}
                                />
                                <Command.Dialog
                                    bind:open={songwriterDialogOpen}
                                    class="bg-slate-900 border border-slate-700"
                                >
                                    <Command.Input
                                        placeholder="Search artists..."
                                        class="border-none bg-slate-900 text-slate-50 placeholder:text-slate-400"
                                        bind:value={artistSearch}
                                    />
                                    <Command.List
                                        class="bg-slate-900 text-slate-50"
                                    >
                                        {#if loadingArtists}
                                            <Command.Empty
                                                class="py-6 text-center text-sm text-slate-400"
                                            >
                                                Loading...
                                            </Command.Empty>
                                        {:else}
                                            <Command.Group class="p-1">
                                                {#each artists as artist (artist.id)}
                                                    <Command.Item
                                                        value={artist.name}
                                                        onSelect={() =>
                                                            handleArtistSelect(
                                                                artist,
                                                                "songwriter",
                                                            )}
                                                        class="cursor-pointer select-none relative text-slate-400 flex items-center rounded-sm px-2 py-1.5 text-sm outline-none aria-selected:bg-slate-700 aria-selected:text-slate-50 data-[disabled]:pointer-events-none data-[disabled]:opacity-50 hover:bg-slate-800"
                                                    >
                                                        {artist.name}
                                                    </Command.Item>
                                                {/each}
                                            </Command.Group>
                                        {/if}
                                    </Command.List>
                                </Command.Dialog>
                            </div>
                        </div>

                        <div class="grid grid-cols-4 items-center gap-4">
                            <Label
                                for="description"
                                class="text-right text-slate-400"
                                >Description</Label
                            >
                            <TextArea
                                rows={6}
                                placeholder="Enter a detailed description..."
                                name="description"
                                classname="col-span-3 bg-slate-800 border-slate-700 text-slate-50"
                            />
                        </div>
                    </div>
                    <div class="flex justify-end mt-4">
                        <Button
                            type="submit"
                            class="bg-slate-800 hover:bg-slate-700 text-slate-50"
                            >Add piece</Button
                        >
                    </div>
                </form></Tabs.Content
            >

            <Tabs.Content value="release">
                <form
                    method="POST"
                    enctype="multipart/form-data"
                    action="?/addrelease"
                    use:enhance={handleSubmit}
                >
                    <div class="grid gap-4 py-4">
                        <div class="grid grid-cols-4 items-center gap-4">
                            <Label for="title" class="text-right text-slate-400"
                                >Release Title*</Label
                            >
                            <Input
                                id="title"
                                name="name"
                                class="col-span-3 bg-slate-800 border-slate-700 text-slate-50"
                                required
                            />
                        </div>

                        <div class="grid grid-cols-4 items-center gap-4">
                            <Label
                                for="performer"
                                class="text-right text-slate-400"
                                >Performer*</Label
                            >
                            <div class="col-span-3">
                                <div class="flex flex-wrap gap-2 mb-2">
                                    {#each selectedPerformers as performer}
                                        <div
                                            class="bg-slate-700 px-2 py-1 rounded-md flex items-center gap-2"
                                        >
                                            {performer.name}
                                            <button
                                                class="text-slate-400 hover:text-slate-200"
                                                on:click={() =>
                                                    handleArtistSelect(
                                                        performer,
                                                        "performer",
                                                    )}>×</button
                                            >
                                        </div>
                                    {/each}
                                </div>
                                <Button
                                    class="bg-slate-800 hover:bg-slate-700 text-slate-50"
                                    on:click={() => {
                                        getArtists("performer");
                                    }}
                                >
                                    Add performer...
                                </Button>
                                <input
                                    type="hidden"
                                    name="performers"
                                    value={selectedPerformers
                                        .map((p) => p.id)
                                        .join(",")}
                                />
                                <Command.Dialog
                                    bind:open={performerDialogOpen}
                                    class="bg-slate-900 border border-slate-700"
                                >
                                    <Command.Input
                                        placeholder="Search artists..."
                                        class="border-none bg-slate-900 text-slate-50 placeholder:text-slate-400"
                                        bind:value={artistSearch}
                                    />
                                    <Command.List
                                        class="bg-slate-900 text-slate-50"
                                    >
                                        {#if loadingArtists}
                                            <Command.Empty
                                                class="py-6 text-center text-sm text-slate-400"
                                            >
                                                Loading...
                                            </Command.Empty>
                                        {:else}
                                            <Command.Group class="p-1">
                                                {#each artists as artist (artist.id)}
                                                    <Command.Item
                                                        value={artist.name}
                                                        onSelect={() =>
                                                            handleArtistSelect(
                                                                artist,
                                                                "performer",
                                                            )}
                                                        class="cursor-pointer select-none relative text-slate-400 flex items-center rounded-sm px-2 py-1.5 text-sm outline-none aria-selected:bg-slate-700 aria-selected:text-slate-50 data-[disabled]:pointer-events-none data-[disabled]:opacity-50 hover:bg-slate-800"
                                                    >
                                                        {artist.name}
                                                    </Command.Item>
                                                {/each}
                                            </Command.Group>
                                        {/if}
                                    </Command.List>
                                </Command.Dialog>
                            </div>
                        </div>

                        <div class="grid grid-cols-4 items-center gap-4">
                            <Label
                                for="description"
                                class="text-right text-slate-400"
                                >Description</Label
                            >
                            <TextArea
                                rows={6}
                                name="description"
                                placeholder="Enter a detailed description..."
                                classname="col-span-3 bg-slate-800 border-slate-700 text-slate-50"
                            />
                        </div>

                        <div class="grid grid-cols-4 items-center gap-4">
                            <Label for="file" class="text-right text-slate-400"
                                >Cover Art</Label
                            >
                            <Input
                                type="file"
                                name="file"
                                id="file"
                                accept="image/*"
                                on:change={handleFileChange}
                                class="col-span-3 bg-slate-800 border-slate-700 text-slate-50 file:bg-slate-700 file:text-slate-50 file:border-0"
                            />
                        </div>
                    </div>
                    <div class="flex justify-end mt-4">
                        <Button
                            type="submit"
                            class="bg-slate-800 hover:bg-slate-700 text-slate-50"
                            >Add release</Button
                        >
                    </div>
                </form></Tabs.Content
            >

            <Tabs.Content value="artist">
                <form
                    method="POST"
                    enctype="multipart/form-data"
                    action="?/addartist"
                    use:enhance={handleSubmit}
                >
                    <div class="grid gap-4 py-4">
                        <div class="grid grid-cols-4 items-center gap-4">
                            <Label for="name" class="text-right text-slate-400"
                                >Artist Name*</Label
                            >
                            <Input
                                id="name"
                                name="name"
                                class="col-span-3 bg-slate-800 border-slate-700 text-slate-50"
                                required
                            />
                        </div>

                        <div class="grid grid-cols-4 items-center gap-4">
                            <Label for="type" class="text-right text-slate-400"
                                >Artist Type*</Label
                            >
                            <select
                                id="type"
                                name="type"
                                class="col-span-3 bg-slate-800 border-slate-700 text-slate-50 rounded-md px-3 py-2"
                                required
                            >
                                <option value="">Select type...</option>
                                <option value="performer">Performer</option>
                                <option value="composer">Composer</option>
                                <option value="songwriter">Songwriter</option>
                            </select>
                        </div>

                        <div class="grid grid-cols-4 items-center gap-4">
                            <Label
                                for="description"
                                class="text-right text-slate-400"
                                >Description</Label
                            >
                            <TextArea
                                rows={6}
                                placeholder="Enter a detailed description..."
                                name="description"
                                classname="col-span-3 bg-slate-800 border-slate-700 text-slate-50"
                            />
                        </div>

                        <div class="grid grid-cols-4 items-center gap-4">
                            <Label for="file" class="text-right text-slate-400"
                                >Image</Label
                            >
                            <Input
                                type="file"
                                name="file"
                                id="artistFile"
                                accept="image/*"
                                on:change={handleFileChange}
                                class="col-span-3 bg-slate-800 border-slate-700 text-slate-50 file:bg-slate-700 file:text-slate-50 file:border-0"
                            />
                        </div>
                    </div>
                    <div class="flex justify-end mt-4">
                        <Button
                            type="submit"
                            class="bg-slate-800 hover:bg-slate-700 text-slate-50"
                            >Add artist</Button
                        >
                    </div>
                </form></Tabs.Content
            >
        </Tabs.Root>
    </Dialog.Content>
</Dialog.Root>
