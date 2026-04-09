(function () {
    'use strict';

    // ===== Theme toggle =====
    const themeToggle = document.getElementById('theme-toggle');
    const body = document.body;
    const saved = localStorage.getItem('theme');
    if (saved) body.setAttribute('data-theme', saved);

    themeToggle.addEventListener('click', () => {
        const current = body.getAttribute('data-theme');
        const next = current === 'dark' ? 'light' : 'dark';
        body.setAttribute('data-theme', next);
        localStorage.setItem('theme', next);
    });

    // ===== Nav scroll state =====
    const nav = document.getElementById('nav');
    const onScroll = () => {
        if (window.scrollY > 8) nav.classList.add('scrolled');
        else nav.classList.remove('scrolled');
    };
    window.addEventListener('scroll', onScroll, { passive: true });
    onScroll();

    // ===== Active link highlighting =====
    const sections = document.querySelectorAll('section[id]');
    const navLinks = document.querySelectorAll('.nav-links a');
    const linkMap = {};
    navLinks.forEach(link => {
        const href = link.getAttribute('href').replace('#', '');
        linkMap[href] = link;
    });
    const activate = () => {
        let current = '';
        const y = window.scrollY + 120;
        sections.forEach(s => {
            if (s.offsetTop <= y) current = s.id;
        });
        navLinks.forEach(l => l.classList.remove('active'));
        if (linkMap[current]) linkMap[current].classList.add('active');
    };
    window.addEventListener('scroll', activate, { passive: true });
    activate();

    // ===== Project filters =====
    const filters = document.querySelectorAll('.filter');
    const cards = document.querySelectorAll('.project-card');
    filters.forEach(btn => {
        btn.addEventListener('click', () => {
            filters.forEach(f => f.classList.remove('active'));
            btn.classList.add('active');
            const f = btn.dataset.filter;
            cards.forEach(c => {
                if (f === 'all') {
                    c.classList.remove('hidden');
                } else {
                    const cats = (c.dataset.categories || '').split(',').map(s => s.trim());
                    if (cats.includes(f)) c.classList.remove('hidden');
                    else c.classList.add('hidden');
                }
            });
        });
    });

    // ===== Reveal on scroll =====
    const revealEls = document.querySelectorAll('.section-head, .about-grid, .timeline-item, .project-card, .skill-group, .contact-card');
    revealEls.forEach(el => el.classList.add('reveal'));
    if ('IntersectionObserver' in window) {
        const io = new IntersectionObserver((entries) => {
            entries.forEach(e => {
                if (e.isIntersecting) {
                    e.target.classList.add('visible');
                    io.unobserve(e.target);
                }
            });
        }, { threshold: 0.1, rootMargin: '0px 0px -40px 0px' });
        revealEls.forEach(el => io.observe(el));
    } else {
        revealEls.forEach(el => el.classList.add('visible'));
    }

    // ===== Year =====
    const yearEl = document.getElementById('year');
    if (yearEl) yearEl.textContent = new Date().getFullYear();

    // ===== Animated number counters =====
    const stats = document.querySelectorAll('.stat-num');
    const animateNum = (el) => {
        const raw = el.textContent.trim();
        const match = raw.match(/^(\d+)(.*)$/);
        if (!match) return;
        const target = parseInt(match[1], 10);
        const suffix = match[2];
        let current = 0;
        const steps = 30;
        const inc = target / steps;
        let i = 0;
        const tick = () => {
            i++;
            current = Math.min(Math.ceil(inc * i), target);
            el.textContent = current + suffix;
            if (i < steps) requestAnimationFrame(tick);
        };
        tick();
    };
    if ('IntersectionObserver' in window) {
        const statObs = new IntersectionObserver((entries) => {
            entries.forEach(e => {
                if (e.isIntersecting) {
                    animateNum(e.target);
                    statObs.unobserve(e.target);
                }
            });
        }, { threshold: 0.5 });
        stats.forEach(s => statObs.observe(s));
    }
})();
