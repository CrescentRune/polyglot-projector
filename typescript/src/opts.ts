import cli from "command-line-args";

export type ProjectorOptions = {
    pwd?: string;
    config?: string;
    args?: string[];
}

export default function getOptions(): ProjectorOptions {
    return cli([
        { name: 'config', type: String },
        { name: 'pwd', type: String },
        { name: 'args', type: String, multiple: true, defaultOption: true }
    ]) as ProjectorOptions;
}
