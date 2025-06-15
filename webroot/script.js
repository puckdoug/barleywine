// Barleywine Static Server - Test JavaScript File
// This file tests MIME type detection for JavaScript files

document.addEventListener('DOMContentLoaded', function() {
    console.log('🍺 Barleywine Static Server - JavaScript loaded successfully!');

    // Add some interactive functionality to test JS execution
    initializeInteractivity();
    addNavigationHelpers();
    createFileTypeDemo();
});

function initializeInteractivity() {
    // Add click animations to cards
    const cards = document.querySelectorAll('.container, .card');
    cards.forEach(card => {
        card.addEventListener('mouseenter', function() {
            this.style.transition = 'transform 0.3s ease';
            this.style.transform = 'scale(1.02)';
        });

        card.addEventListener('mouseleave', function() {
            this.style.transform = 'scale(1)';
        });
    });

    // Add ripple effect to buttons
    const buttons = document.querySelectorAll('.nav-link, .button');
    buttons.forEach(button => {
        button.addEventListener('click', function(e) {
            const ripple = document.createElement('span');
            const rect = this.getBoundingClientRect();
            const size = Math.max(rect.width, rect.height);
            const x = e.clientX - rect.left - size / 2;
            const y = e.clientY - rect.top - size / 2;

            ripple.style.cssText = `
                position: absolute;
                width: ${size}px;
                height: ${size}px;
                left: ${x}px;
                top: ${y}px;
                background: rgba(255,255,255,0.5);
                border-radius: 50%;
                transform: scale(0);
                animation: ripple 0.6s linear;
                pointer-events: none;
            `;

            this.style.position = 'relative';
            this.style.overflow = 'hidden';
            this.appendChild(ripple);

            setTimeout(() => {
                ripple.remove();
            }, 600);
        });
    });
}

function addNavigationHelpers() {
    // Add a back-to-top button if page is long enough
    if (document.body.scrollHeight > window.innerHeight + 100) {
        const backToTop = document.createElement('div');
        backToTop.innerHTML = '↑';
        backToTop.style.cssText = `
            position: fixed;
            bottom: 20px;
            right: 20px;
            width: 50px;
            height: 50px;
            background: #007acc;
            color: white;
            border-radius: 50%;
            display: flex;
            align-items: center;
            justify-content: center;
            cursor: pointer;
            font-size: 20px;
            font-weight: bold;
            opacity: 0;
            transition: opacity 0.3s ease;
            z-index: 1000;
        `;

        document.body.appendChild(backToTop);

        window.addEventListener('scroll', function() {
            if (window.scrollY > 200) {
                backToTop.style.opacity = '1';
            } else {
                backToTop.style.opacity = '0';
            }
        });

        backToTop.addEventListener('click', function() {
            window.scrollTo({ top: 0, behavior: 'smooth' });
        });
    }

    // Add keyboard shortcuts
    document.addEventListener('keydown', function(e) {
        // Alt + H for home
        if (e.altKey && e.key === 'h') {
            window.location.href = '/';
        }

        // Alt + S for subdir
        if (e.altKey && e.key === 's') {
            window.location.href = '/subdir/';
        }
    });
}

function createFileTypeDemo() {
    // Create a demo section showing different file types
    const demoSection = document.createElement('div');
    demoSection.style.cssText = `
        position: fixed;
        top: 10px;
        right: 10px;
        background: rgba(0,0,0,0.8);
        color: white;
        padding: 15px;
        border-radius: 8px;
        font-size: 12px;
        max-width: 200px;
        z-index: 1001;
    `;

    const title = document.createElement('h4');
    title.textContent = 'File Types Served:';
    title.style.margin = '0 0 10px 0';
    demoSection.appendChild(title);

    const fileTypes = [
        { ext: '.html', mime: 'text/html', example: '/' },
        { ext: '.css', mime: 'text/css', example: '/styles.css' },
        { ext: '.js', mime: 'application/javascript', example: '/script.js' },
        { ext: '.txt', mime: 'text/plain', example: '/test.txt' }
    ];

    fileTypes.forEach(type => {
        const item = document.createElement('div');
        item.style.marginBottom = '5px';
        item.innerHTML = `
            <strong>${type.ext}</strong><br>
            <em>${type.mime}</em><br>
            <a href="${type.example}" style="color: #66ccff; font-size: 10px;">${type.example}</a>
        `;
        demoSection.appendChild(item);
    });

    // Add close button
    const closeBtn = document.createElement('span');
    closeBtn.innerHTML = '×';
    closeBtn.style.cssText = `
        position: absolute;
        top: 5px;
        right: 8px;
        cursor: pointer;
        font-size: 16px;
        font-weight: bold;
    `;
    closeBtn.addEventListener('click', function() {
        demoSection.remove();
    });
    demoSection.appendChild(closeBtn);

    // Only show demo on main page
    if (window.location.pathname === '/') {
        document.body.appendChild(demoSection);

        // Auto-hide after 10 seconds
        setTimeout(() => {
            if (demoSection.parentNode) {
                demoSection.style.opacity = '0';
                demoSection.style.transition = 'opacity 0.5s ease';
                setTimeout(() => demoSection.remove(), 500);
            }
        }, 10000);
    }
}

// Add CSS animations for ripple effect
const style = document.createElement('style');
style.textContent = `
    @keyframes ripple {
        to {
            transform: scale(4);
            opacity: 0;
        }
    }
`;
document.head.appendChild(style);

// Export functions for potential use in other scripts
window.BarleywineUtils = {
    initializeInteractivity,
    addNavigationHelpers,
    createFileTypeDemo
};

// Test fetch functionality to demonstrate AJAX requests work
function testServerConnectivity() {
    fetch('/styles.css')
        .then(response => {
            console.log('✅ CSS file fetch test successful:', response.status, response.headers.get('content-type'));
        })
        .catch(error => {
            console.error('❌ CSS file fetch test failed:', error);
        });
}

// Run connectivity test
testServerConnectivity();

console.log('📁 Available keyboard shortcuts:');
console.log('  Alt + H: Go to home');
console.log('  Alt + S: Go to subdirectory');
