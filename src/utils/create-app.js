import { createAppTaskCloneTemplate } from './create-app-task-clone-template';
import { createAppTaskInitializeGit } from './create-app-task-initialize-git';
import { createAppTaskInstallDependencies } from './create-app-task-install-dependencies';
import { createAppTaskRunInitScript } from './create-app-task-run-init-script';
import { tasks } from './vendor/clack-tasks';
export async function createApp(args) {
    return tasks([
        // Clone the template to the target directory
        createAppTaskCloneTemplate(args),
        // Install the dependencies
        createAppTaskInstallDependencies(args),
        // Run the init script define in package.json .init property
        createAppTaskRunInitScript(args),
        // Initialize git repository
        createAppTaskInitializeGit(args),
    ]);
}
