import { ProjectorOptions } from './opts';

import path from "path";

export enum Operation {
    Print,
    Add,
    Remove,
}

export type Config = {
    args: string[],
    operation: Operation,
    config: string,
    pwd: string,
}

function getPwd(opts: ProjectorOptions): string {
    if (opts.pwd) {
        return opts.pwd;
    }

    return process.cwd();
}

function getConfig(opts: ProjectorOptions) {
    if (opts.config) {
        return opts.config;
    }


    const home = process.env["HOME"];
    const loc = "~/Library/Preferences/" || home;
    if (!loc && !home) {
       throw new Error("unable to determine config location"); 
    }

    if (loc === home) {
        return path.join(loc, ".projector.json"); 
    }

    return path.join(loc, "projector", "projector.json");

}

function getOperation(opts: ProjectorOptions) {
    if (!opts.args || opts.args.length === 0) {
        return Operation.Print;
    }

    if (opts[0] === 'add') {
        return Operation.Add;
    } else if (opts[0] === 'delete') {
        return Operation.Remove;
    }

    return Operation.Print;
}

function getArgs(opts: ProjectorOptions) {
    if (!opts.args || opts.args.length === 0) {
        return [];
    }

    const operation = getOperation(opts);
    if (operation === Operation.Print) {
        if (opts.args.length > 1) {
            throw new Error(`Expected 0 or 1 arg(s) and got ${opts.args.length}`);
        }
        return opts.args;
    }

    if (operation === Operation.Add) {
        if (opts.args.length !== 3) {
            throw new Error(`Expected 2 args and got ${opts.args.length}`);
        }

        return opts.args.slice(1);
    }

    if (opts.args.length !== 2) {
        throw new Error(`Expected 1 args and got ${opts.args.length}`);
    }

    return opts.args.slice(1);
}


export default function config(opts: ProjectorOptions) {
    return {
        pwd: getPwd(opts),
        config: getConfig(opts),
        args: getArgs(opts),
        operation: getOperation(opts),
}


