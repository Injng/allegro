<script lang="ts">
    import { cn } from "$lib/utils"; // assuming you have this utility function

    let description = "";

    export let rows = 4; // default number of rows
    export let placeholder = "enter description...";
    export let classname = ""; // allow passing additional classes
</script>

<textarea
    {rows}
    {placeholder}
    bind:value={description}
    class={cn(
        "flex min-h-[80px] w-full rounded-md border border-slate-700 bg-slate-800 px-3 py-2 text-sm text-slate-50 placeholder:text-slate-400",
        "focus:outline-none focus:ring-2 focus:ring-slate-600 focus:ring-offset-2 focus:ring-offset-slate-900",
        "disabled:cursor-not-allowed disabled:opacity-50",
        classname,
    )}
/>
