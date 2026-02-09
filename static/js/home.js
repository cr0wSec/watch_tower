/**
/ * Watch Tower - Home Page JavaScript
 * Global Monitoring System
 */

// =============================================================================
// FEED TABS
// =============================================================================

function initFeedTabs() {
    document.querySelectorAll('.feed-tab').forEach(tab => {
        tab.addEventListener('click', () => {
            // Remove active state from all tabs
            document.querySelectorAll('.feed-tab').forEach(t => {
                t.classList.remove('active', 'text-white', 'border-b-2', 'border-blue-500');
                t.classList.add('text-gray-500');
            });

            // Add active state to clicked tab
            tab.classList.add('active', 'text-white', 'border-b-2', 'border-blue-500');
            tab.classList.remove('text-gray-500');

            // Show corresponding content
            const tabName = tab.dataset.tab;
            document.querySelectorAll('.feed-content').forEach(content => {
                content.classList.add('hidden');
                if (content.dataset.tab === tabName) {
                    content.classList.remove('hidden');
                }
            });
        });
    });
}

function initFeedItems() {
    document.querySelectorAll('.feed-content').forEach(feedContent => {
        const items = feedContent.querySelectorAll('.feed-item');
        const details = feedContent.querySelectorAll('.feed-detail');

        items.forEach(item => {
            item.addEventListener('click', () => {
                const index = item.dataset.index;

                // Remove active state from all items in this feed
                items.forEach(i => {
                    i.classList.remove('active', 'bg-gray-900/50');
                });

                // Add active state to clicked item
                item.classList.add('active', 'bg-gray-900/50');

                // Hide all details and show the selected one
                details.forEach(detail => {
                    if (detail.dataset.index === index) {
                        detail.classList.remove('hidden');
                    } else {
                        detail.classList.add('hidden');
                    }
                });
            });
        });

        // Select first item by default (already done in HTML, but ensure JS state is correct)
        const firstItem = feedContent.querySelector('.feed-item[data-index="0"]');
        if (firstItem) {
            firstItem.classList.add('active', 'bg-gray-900/50');
        }
    });
}

// =============================================================================
// LAST UPDATED TIMESTAMPS
// =============================================================================

const lastUpdatedTimestamps = {
    indices: new Date(Date.now() - 5 * 60 * 1000),
    crypto: new Date(Date.now() - 3 * 60 * 1000),
    commodities: new Date(Date.now() - 8 * 60 * 1000)
};

function formatRelativeTime(date) {
    const diffMins = Math.floor((new Date() - date) / 60000);
    if (diffMins < 1) return 'Just now';
    if (diffMins === 1) return '1m ago';
    if (diffMins < 60) return `${diffMins}m ago`;
    const diffHours = Math.floor(diffMins / 60);
    if (diffHours < 24) return `${diffHours}h ago`;
    return `${Math.floor(diffHours / 24)}d ago`;
}

function updateLastUpdatedDisplays() {
    document.querySelectorAll('.last-updated').forEach(el => {
        const section = el.dataset.section;
        const timestamp = lastUpdatedTimestamps[section];
        if (timestamp) el.textContent = formatRelativeTime(timestamp);
    });
}

// =============================================================================
// WORLD CLOCKS
// =============================================================================

function updateClocks() {
    const now = new Date();
    const timeOptions = { hour: '2-digit', minute: '2-digit', hour12: false };

    document.getElementById('clock-utc').textContent =
        now.toLocaleTimeString('en-GB', { ...timeOptions, timeZone: 'UTC' });
    document.getElementById('clock-dc').textContent =
        now.toLocaleTimeString('en-US', { ...timeOptions, timeZone: 'America/New_York' });
    document.getElementById('clock-london').textContent =
        now.toLocaleTimeString('en-GB', { ...timeOptions, timeZone: 'Europe/London' });
    document.getElementById('clock-moscow').textContent =
        now.toLocaleTimeString('ru-RU', { ...timeOptions, timeZone: 'Europe/Moscow' });
    document.getElementById('clock-beijing').textContent =
        now.toLocaleTimeString('zh-CN', { ...timeOptions, timeZone: 'Asia/Shanghai' });
    document.getElementById('clock-local').textContent =
        now.toLocaleTimeString('en-GB', timeOptions);
}


// =============================================================================
// GLOBE UTILITIES
// =============================================================================

function getPolygonCenter(geometry) {
    let coords = [];
    if (geometry.type === 'Polygon') {
        coords = geometry.coordinates[0];
    } else if (geometry.type === 'MultiPolygon') {
        let maxLen = 0;
        geometry.coordinates.forEach(poly => {
            if (poly[0].length > maxLen) {
                maxLen = poly[0].length;
                coords = poly[0];
            }
        });
    }
    const sum = coords.reduce((acc, c) => ({
        lng: acc.lng + c[0],
        lat: acc.lat + c[1]
    }), { lng: 0, lat: 0 });
    return {
        lat: sum.lat / coords.length,
        lng: sum.lng / coords.length
    };
}

function pipelineToGeoJSON(pipeline) {
    return {
        type: 'Feature',
        properties: { name: pipeline.name },
        geometry: {
            type: 'LineString',
            coordinates: pipeline.coordinates
        }
    };
}

// =============================================================================
// MAP MARKERS & TOOLTIPS
// =============================================================================

const MARKER_COLORS = {
    military_base: '#3b82f6',
    strait: '#8b5cf6',
    alert_critical: '#ef4444',
    alert_high: '#f97316',
    alert_medium: '#eab308',
    alert_low: '#22c55e'
};

// Fake alert data for demonstration
const ALERT_DATA = [
    {
        name: 'Earthquake M6.2',
        lat: 37.4,
        lng: 136.9,
        severity: 'CRITICAL',
        severityColor: 'bg-red-500',
        description: 'Major seismic event detected. Tsunami warning issued.',
        time: '12 minutes ago',
        type: 'alert_critical'
    },
    {
        name: 'Cyber Attack - EU Infrastructure',
        lat: 50.8,
        lng: 4.3,
        severity: 'HIGH',
        severityColor: 'bg-orange-500',
        description: 'Coordinated attack on power grid systems.',
        time: '45 minutes ago',
        type: 'alert_high'
    },
    {
        name: 'GPS Jamming Detected',
        lat: 55.0,
        lng: 20.0,
        severity: 'MEDIUM',
        severityColor: 'bg-yellow-500',
        description: 'Navigation interference in Baltic Sea region.',
        time: '2 hours ago',
        type: 'alert_medium'
    },
    {
        name: 'Military Exercise',
        lat: 15.5,
        lng: 115.0,
        severity: 'HIGH',
        severityColor: 'bg-orange-500',
        description: 'Large-scale naval exercise in South China Sea.',
        time: '6 hours ago',
        type: 'alert_high'
    },
    {
        name: 'Wildfire - California',
        lat: 34.0,
        lng: -118.5,
        severity: 'HIGH',
        severityColor: 'bg-orange-500',
        description: '12,000 acres burning. 20% contained.',
        time: '2 hours ago',
        type: 'alert_high'
    },
    {
        name: 'Pipeline Disruption',
        lat: 51.5,
        lng: 30.5,
        severity: 'MEDIUM',
        severityColor: 'bg-yellow-500',
        description: 'Gas flow interruption reported.',
        time: '4 hours ago',
        type: 'alert_medium'
    },
    {
        name: 'Drone Activity',
        lat: 26.0,
        lng: 56.0,
        severity: 'HIGH',
        severityColor: 'bg-orange-500',
        description: 'Unidentified drones near Strait of Hormuz.',
        time: '1 hour ago',
        type: 'alert_high'
    },
    {
        name: 'Flooding Emergency',
        lat: -27.5,
        lng: 153.0,
        severity: 'MEDIUM',
        severityColor: 'bg-yellow-500',
        description: 'Emergency evacuations underway in Queensland.',
        time: '5 hours ago',
        type: 'alert_medium'
    }
];

// Special zones to highlight
const SPECIAL_ZONES = [
    { lat: 15.5, lng: 115.0, name: 'South China Sea', color: 'rgba(239, 68, 68, 0.3)', radius: 8 },
    { lat: 55.0, lng: 20.0, name: 'Baltic Sea', color: 'rgba(249, 115, 22, 0.3)', radius: 5 },
    { lat: 26.5, lng: 56.3, name: 'Strait of Hormuz', color: 'rgba(234, 179, 8, 0.3)', radius: 3 },
    { lat: 31.5, lng: 32.3, name: 'Suez Canal', color: 'rgba(234, 179, 8, 0.3)', radius: 2 },
    { lat: 50.0, lng: 35.0, name: 'Ukraine Conflict Zone', color: 'rgba(239, 68, 68, 0.4)', radius: 6 },
    { lat: 37.5, lng: 137.0, name: 'Japan Earthquake Zone', color: 'rgba(239, 68, 68, 0.3)', radius: 4 }
];

function showTooltip(type, data, event) {
    const tooltip = document.getElementById('marker-tooltip');
    const tooltipFlag = document.getElementById('tooltip-flag');
    const tooltipName = document.getElementById('tooltip-name');
    const tooltipContent = document.getElementById('tooltip-content');

    if (!tooltip || !event) return;

    let html = '';

    if (type === 'military_base') {
        tooltipFlag.textContent = '🎖️';
        tooltipFlag.classList.remove('hidden');
        tooltipName.textContent = data.name || 'Military Base';
        html = `
            <div class="grid grid-cols-2 gap-x-3 gap-y-1">
                <div class="text-gray-500">Country</div><div class="text-gray-300">${data.country || 'N/A'}</div>
                <div class="text-gray-500">Operator</div><div class="text-gray-300">${data.operator || 'N/A'}</div>
                <div class="text-gray-500">Type</div><div class="text-gray-300">${data.base_type || 'Military Base'}</div>
            </div>
        `;
    } else if (type === 'strait') {
        tooltipFlag.textContent = '🌊';
        tooltipFlag.classList.remove('hidden');
        tooltipName.textContent = data.name || 'Strait';
        html = `
            <div class="grid grid-cols-2 gap-x-3 gap-y-1">
                <div class="text-gray-500">Type</div><div class="text-gray-300">${data.strait_type || 'Strait'}</div>
                <div class="text-gray-500">Importance</div><div class="text-gray-300">${data.importance || 'Strategic chokepoint'}</div>
            </div>
        `;
    } else if (type === 'alert') {
        tooltipFlag.textContent = '⚠️';
        tooltipFlag.classList.remove('hidden');
        tooltipName.textContent = data.name || 'Alert';
        html = `
            <div class="space-y-1">
                <div class="flex items-center gap-2">
                    <span class="w-2 h-2 rounded-full ${data.severityColor || 'bg-red-500'} animate-pulse"></span>
                    <span class="${(data.severityColor || 'bg-red-500').replace('bg-', 'text-')}">${data.severity || 'ALERT'}</span>
                </div>
                <div class="text-gray-400">${data.description || ''}</div>
                <div class="text-gray-500 text-[10px]">${data.time || ''}</div>
            </div>
        `;
    }

    tooltipContent.innerHTML = html;
    tooltip.classList.remove('hidden');

    // Position tooltip
    let left = event.clientX + 15;
    let top = event.clientY + 15;

    // Prevent tooltip from going off-screen
    if (left + 250 > window.innerWidth) {
        left = event.clientX - 260;
    }
    if (top + 150 > window.innerHeight) {
        top = event.clientY - 160;
    }

    tooltip.style.left = left + 'px';
    tooltip.style.top = top + 'px';
}

function hideTooltip() {
    const tooltip = document.getElementById('marker-tooltip');
    tooltip.classList.add('hidden');
}

function createMarker(type, data) {
    const el = document.createElement('div');

    // Check if it's an alert type
    const isAlert = type.startsWith('alert_');
    const color = MARKER_COLORS[type] || '#3b82f6';

    if (isAlert) {
        // Create pulsing alert marker
        el.style.cssText = `
            position: relative;
            width: 24px;
            height: 24px;
            margin-left: -12px;
            margin-top: -12px;
            cursor: pointer;
            pointer-events: auto;
        `;
        el.innerHTML = `
            <div style="
                position: absolute;
                width: 100%;
                height: 100%;
                border-radius: 50%;
                background: ${color};
                opacity: 0.4;
                animation: pulse-ring 1.5s ease-out infinite;
                pointer-events: none;
            "></div>
            <div style="
                position: absolute;
                width: 100%;
                height: 100%;
                border-radius: 50%;
                background: ${color};
                opacity: 0.4;
                animation: pulse-ring 1.5s ease-out infinite 0.5s;
                pointer-events: none;
            "></div>
            <div style="
                position: absolute;
                top: 50%;
                left: 50%;
                transform: translate(-50%, -50%);
                width: 12px;
                height: 12px;
                border-radius: 50%;
                background: ${color};
                border: 2px solid white;
                box-shadow: 0 0 10px ${color};
                pointer-events: none;
            "></div>
        `;
    } else {
        // Regular marker (military base, strait)
        el.style.cssText = `
            width: 14px;
            height: 14px;
            margin-left: -7px;
            margin-top: -7px;
            border-radius: 50%;
            background-color: ${color};
            border: 2px solid white;
            cursor: pointer;
            box-shadow: 0 0 8px ${color};
            pointer-events: auto;
        `;
    }

    // Use addEventListener for better event handling
    el.addEventListener('mouseenter', (e) => {
        e.stopPropagation();
        showTooltip(isAlert ? 'alert' : type, data, e);
    });
    el.addEventListener('mouseleave', (e) => {
        e.stopPropagation();
        hideTooltip();
    });
    el.addEventListener('mousemove', (e) => {
        e.stopPropagation();
    });

    return el;
}

// Add CSS animation for pulsing alerts
const style = document.createElement('style');
style.textContent = `
    @keyframes pulse-ring {
        0% {
            transform: scale(0.5);
            opacity: 0.6;
        }
        100% {
            transform: scale(2);
            opacity: 0;
        }
    }
`;
document.head.appendChild(style);

function prepareMarkers(bases, straits, alerts) {
    return [
        ...bases.map(b => ({
            lat: b.coordinates[1],
            lng: b.coordinates[0],
            type: 'military_base',
            data: b
        })),
        ...straits.map(s => ({
            lat: s.coordinates[1],
            lng: s.coordinates[0],
            type: 'strait',
            data: s
        })),
        ...alerts.map(a => ({
            lat: a.lat,
            lng: a.lng,
            type: a.type,
            data: a
        }))
    ];
}

// =============================================================================
// GLOBE INITIALIZATION
// =============================================================================

let globe = null;
let hoveredPolygon = null;
let allLayers = null;

const layerVisibility = {
    pipelines: true,
    military_bases: true,
    straits: true,
    alerts: true
};

function resizeGlobe() {
    if (globe) {
        const container = document.getElementById('globe-container');
        globe.width(container.clientWidth).height(container.clientHeight);
    }
}

function updateMarkers() {
    if (!globe || !allLayers) return;
    globe.htmlElementsData(prepareMarkers(
        layerVisibility.military_bases ? allLayers.military_bases : [],
        layerVisibility.straits ? allLayers.straits : [],
        layerVisibility.alerts ? ALERT_DATA : []
    ));
}

function updatePipelines() {
    if (!globe || !allLayers) return;
    globe.pathsData(
        layerVisibility.pipelines ? allLayers.pipelines.map(pipelineToGeoJSON) : []
    );
}

function initLayerToggles() {
    document.getElementById('toggle-alerts').addEventListener('change', (e) => {
        layerVisibility.alerts = e.target.checked;
        updateMarkers();
        // Also toggle the rings (special zones)
        if (globe) {
            globe.ringsData(e.target.checked ? SPECIAL_ZONES : []);
        }
    });

    document.getElementById('toggle-pipelines').addEventListener('change', (e) => {
        layerVisibility.pipelines = e.target.checked;
        updatePipelines();
    });

    document.getElementById('toggle-military-bases').addEventListener('change', (e) => {
        layerVisibility.military_bases = e.target.checked;
        updateMarkers();
    });

    document.getElementById('toggle-straits').addEventListener('change', (e) => {
        layerVisibility.straits = e.target.checked;
        updateMarkers();
    });
}

function initGlobe() {
    const globeContainer = document.getElementById('globe-container');

    Promise.all([
        fetch('https://unpkg.com/world-atlas@2/countries-110m.json').then(r => r.json()),
        fetch('/api/layers').then(r => r.json())
    ]).then(([topology, layers]) => {
        allLayers = layers;
        const countries = topojson.feature(topology, topology.objects.countries);

        globe = Globe()
            .width(globeContainer.clientWidth)
            .height(globeContainer.clientHeight)
            // Pipelines
            .pathsData(layers.pipelines.map(pipelineToGeoJSON))
            .pathPoints(d => d.geometry.coordinates)
            .pathPointLat(p => p[1])
            .pathPointLng(p => p[0])
            .pathColor(() => '#ff9900')
            .pathStroke(3)
            .pathDashLength(0.5)
            .pathDashGap(0.1)
            .pathDashAnimateTime(2000)
            // Special zones (rings)
            .ringsData(SPECIAL_ZONES)
            .ringLat(d => d.lat)
            .ringLng(d => d.lng)
            .ringColor(d => d.color)
            .ringMaxRadius(d => d.radius)
            .ringPropagationSpeed(2)
            .ringRepeatPeriod(1000)
            // Markers (military bases, straits, and alerts)
            .htmlElementsData(prepareMarkers(layers.military_bases, layers.straits, ALERT_DATA))
            .htmlLat(d => d.lat)
            .htmlLng(d => d.lng)
            .htmlElement(d => createMarker(d.type, d.data))
            // Globe appearance
            .globeImageUrl('//unpkg.com/three-globe/example/img/earth-dark.jpg')
            .showAtmosphere(false)
            .backgroundColor('#000000')
            // Countries
            .polygonsData(countries.features)
            .polygonCapColor(d => d === hoveredPolygon ? 'rgba(120,120,120,0.8)' : 'rgba(60,60,60,0.6)')
            .polygonSideColor(() => 'rgba(60,60,60,0.1)')
            .polygonStrokeColor(() => '#fff')
            .polygonsTransitionDuration(0)
            .polygonLabel(({ properties: p }) => `<b>${p.name}</b>`)
            // Interactions
            .onPolygonHover((p) => {
                hoveredPolygon = p;
                globe.polygonCapColor(globe.polygonCapColor());
                document.body.style.cursor = p ? 'pointer' : 'default';
            })
            (document.getElementById('globe'));
    });
}

// =============================================================================
// INITIALIZATION
// =============================================================================

document.addEventListener('DOMContentLoaded', () => {
    // Initialize UI components
    initFeedTabs();
    initFeedItems();
    initLayerToggles();

    // Start clocks
    updateClocks();
    setInterval(updateClocks, 1000);

    // Start last updated displays
    updateLastUpdatedDisplays();
    setInterval(updateLastUpdatedDisplays, 60000);

    // Initialize globe
    initGlobe();

    // Handle window resize
    window.addEventListener('resize', resizeGlobe);
});
