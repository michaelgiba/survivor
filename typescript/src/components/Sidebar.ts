export class Sidebar {
    private static instance: Sidebar;
    private element: HTMLElement;

    constructor() {
        if (Sidebar.instance) {
            return Sidebar.instance;
        }
        Sidebar.instance = this;

        this.element = document.createElement('div');
        this.element.className = 'sidebar';
        document.body.appendChild(this.element);
    }

    static getInstance(): Sidebar {
        return Sidebar.instance;
    }
}