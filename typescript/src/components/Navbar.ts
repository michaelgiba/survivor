export class Navbar {
    private static instance: Navbar;
    private element: HTMLElement;
    private rolloutSelector: HTMLSelectElement;

    constructor(onRolloutSelect: (filename: string) => void) {
        if (Navbar.instance) {
            return Navbar.instance;
        }
        Navbar.instance = this;

        this.element = document.createElement('nav');
        this.element.className = 'navbar';
        document.body.appendChild(this.element);

        this.rolloutSelector = document.createElement('select');
        this.rolloutSelector.id = 'rolloutSelector';
        
        this.element.innerHTML = `
            <div>Survivor</div>
            ${this.rolloutSelector.outerHTML}
        `;

        // Re-get the element after innerHTML
        this.rolloutSelector = document.getElementById('rolloutSelector') as HTMLSelectElement;
        this.rolloutSelector.addEventListener('change', (e) => {
            const target = e.target as HTMLSelectElement;
            if (target.value) {
                onRolloutSelect(target.value);
            }
        });

        this.loadRollouts();
    }

    private async loadRollouts() {
        try {
            const response = await fetch('/rollouts/');
            const files = await response.json();
            
            this.rolloutSelector.innerHTML = `
                <option value="">Select a rollout...</option>
                ${files.map((file: string) => `
                    <option value="${file}">${file}</option>
                `).join('')}
            `;
        } catch (error) {
            console.error('Error loading rollouts:', error);
            this.rolloutSelector.innerHTML = '<option value="">Error loading rollouts</option>';
        }
    }

    static getInstance(): Navbar {
        return Navbar.instance;
    }
}