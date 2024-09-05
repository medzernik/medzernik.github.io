var graf = [
    {
        label: "A",
        x: 452,
        y: 323,
    },
    {
        label: "B",
        x: 670,
        y: 178,
    },
    {
        label: "C",
        x: 115,
        y: 490,
    },
    {
        label: "D",
        x: 825,
        y: 356,
    },
    {
        label: "E",
        x: 305,
        y: 512,
    },
    {
        label: "F",
        x: 949,
        y: 72,
    },
    {
        label: "G",
        x: 238,
        y: 267,
    },
    {
        label: "H",
        x: 580,
        y: 401,
    },
    {
        label: "I",
        x: 790,
        y: 135,
    },
    {
        label: "J",
        x: 334,
        y: 520,
    },
];
var edges = {
    vertices: ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J"],
    edges: [
        { from: "A", to: "C", weight: 45 }, // Hrana mezi A a B
        { from: "A", to: "E", weight: 67 }, // Hrana mezi B a C
        { from: "A", to: "G", weight: 23 }, // Hrana mezi B a D
        { from: "A", to: "J", weight: 34 }, // Hrana mezi E a F
        { from: "C", to: "E", weight: 89 }, // Hrana mezi D a E
        { from: "B", to: "D", weight: 34 }, // Hrana mezi E a F
        { from: "B", to: "I", weight: 72 }, // Hrana mezi E a G
        { from: "B", to: "H", weight: 56 }, // Hrana mezi E a H
        { from: "H", to: "J", weight: 49 }, // Hrana mezi E a H
        { from: "I", to: "D", weight: 91 }, // Hrana mezi E a I
        { from: "I", to: "F", weight: 37 }, // Hrana mezi G a J
    ],
};
// Nacte canvas z HTML
function LoadCanvas() {
    var canvas = document.getElementById("canvas");
    if (canvas === null || canvas === void 0 ? void 0 : canvas.getContext) {
        var ctx = canvas === null || canvas === void 0 ? void 0 : canvas.getContext("2d");
        if (!ctx) {
            console.error("nemožnost zařídit context2D pro canvas, funkce končí");
            return;
        }
        ctx.clearRect(0, 0, canvas.width, canvas.height);
        DrawGraph(ctx);
        DisplayPath(ctx);
    }
}
// Vykrseleni grafu
function DrawGraph(ctx) {
    // Zakladni nastaveni vizualu grafu s designem textu
    ctx.beginPath();
    ctx.strokeStyle = "#000000";
    ctx.font = "bold 16px Arial";
    ctx.textAlign = "center";
    ctx.textBaseline = "middle";
    var _loop_1 = function (i) {
        ctx.fillStyle = "blue";
        // Nastaveni zacatecniho a konecneho cile
        var originNode = graf.find(function (item) { return edges.edges[i].from === item.label; });
        var destNode = graf.find(function (item) { return edges.edges[i].to === item.label; });
        if (!originNode || !destNode) {
            console.error("Zvolený uzel neexistuje!");
            return { value: void 0 };
        }
        // Vykresleni zacatku a jeho bodu s pismenem
        ctx.moveTo(originNode.x, originNode === null || originNode === void 0 ? void 0 : originNode.y);
        ctx.fillText(originNode.label, originNode.x, originNode.y + 20);
        ctx.fillRect(originNode.x - 7, originNode.y + 7, 10, -10);
        // Vykresleni konce a jeho bodu s pismenem
        ctx.lineTo(destNode.x, destNode.y);
        ctx.fillText(destNode.label, destNode.x, destNode.y + 20);
        ctx.fillRect(destNode.x - 7, destNode.y + 7, 10, -10);
        // Nakresleni cary od zaccatku po konec
        ctx.stroke();
        // Nakresleni vahy
        var wX = (originNode.x + destNode.x) / 2;
        var wY = (originNode.y + destNode.y) / 2;
        ctx.fillStyle = "red";
        for (var i_1 = 0; i_1 < edges.edges.length; ++i_1) {
            if (edges.edges[i_1].from === (originNode === null || originNode === void 0 ? void 0 : originNode.label) &&
                edges.edges[i_1].to === (destNode === null || destNode === void 0 ? void 0 : destNode.label)) {
                ctx.fillText(edges.edges[i_1].weight.toString(), wX, wY);
            }
        }
    };
    for (var i = 0; i < edges.edges.length; ++i) {
        var state_1 = _loop_1(i);
        if (typeof state_1 === "object")
            return state_1.value;
    }
}
// Zobrazeni nejrychlejsi cesty (od zacatku po konec)
function DisplayPath(ctx) {
    // Nacteni dat z form na strance
    var startElement = document.getElementById("start");
    var endElement = document.getElementById("end");
    // Konverze na String
    var start = startElement === null || startElement === void 0 ? void 0 : startElement.value.toUpperCase();
    var end = endElement === null || endElement === void 0 ? void 0 : endElement.value.toUpperCase();
    // Detekce chyby, jestli hodnoty neexistuje, doslo k chybe
    if (!start || !end) {
        console.error("Neexistující uzel!");
        return;
    }
    // Jestli je zadana hodnota delsi nez jeden znak nebo neni pismeno, tak detekujeme chybu
    if (start.length != 1 ||
        end.length != 1 ||
        !isNaN(Number(start)) ||
        !isNaN(Number(end))) {
        console.error("Nesprávna hodnota, zadejte jedno písmeno");
        return;
    }
    // Vypocteni a nalezeni vzdalenosti
    var path = CaculateDjikstra(start, end);
    // Prekresleni linie
    ctx.beginPath();
    var _loop_2 = function (i) {
        ctx.strokeStyle = "#00ff55";
        // Nastaveni zacatecniho a konecneho cile
        var originNode = graf.find(function (item) { return path[i] === item.label; });
        var destNode = graf.find(function (item) { return path[i + 1] === item.label; });
        if (!originNode || !destNode) {
            console.error("Body neexistují");
            return { value: void 0 };
        }
        // Vykresleni zacatku a jeho bodu s pismenem
        ctx.moveTo(originNode.x, originNode.y);
        ctx.fillText(originNode.label, originNode.x, originNode.y + 20);
        ctx.fillRect(originNode.x - 7, originNode.y + 7, 10, -10);
        // Vykresleni konce a jeho bodu s pismenem
        ctx.lineTo(destNode.x, destNode.y);
        ctx.fillText(destNode.label, destNode.x, destNode.y + 20);
        ctx.fillRect(destNode.x - 7, destNode.y + 7, 10, -10);
        // Nakresleni cary od zaccatku po konec
        ctx.stroke();
    };
    for (var i = 0; i < path.length; ++i) {
        var state_2 = _loop_2(i);
        if (typeof state_2 === "object")
            return state_2.value;
    }
}
function CaculateDjikstra(start, end) {
    var queue = [];
    var previous = {};
    var distances = {};
    for (var node in edges.vertices) {
        distances[edges.vertices[node]] = Infinity;
        previous[edges.vertices[node]] = null;
        queue.push(edges.vertices[node]);
    }
    distances[start] = 0;
    while (queue.length > 0) {
        // Na ziskani nejkratsi vzdalenosti muzeme udelat sort
        queue.sort(function (a, b) { return distances[a] - distances[b]; });
        var currentNode = queue.shift();
        // Pokud najdeme cil, funkce vraci cestu
        if (currentNode == end) {
            var path = [];
            var temp = currentNode;
            // Smazani posledni hodnoty na ziskani cesty
            while (temp) {
                path.unshift(temp);
                temp = previous[temp];
            }
            return path;
        }
        // Pokud by neexistovala cesta, vracime chybu
        if (!currentNode) {
            console.error("Cesta neexistuje!");
            return [];
        }
        var neighbors = GetNeighbour(currentNode);
        // Prohledani sousedu
        for (var i = 0; i < neighbors.length; ++i) {
            var alt = distances[currentNode] + neighbors[i][1];
            // Pokud existuje kratsi cesta od souseda, pridat a pokracovat
            if (alt < distances[neighbors[i][0]]) {
                distances[neighbors[i][0]] = alt;
                previous[neighbors[i][0]] = currentNode;
            }
        }
    }
    console.warn("Neexistuje žádná validní cesta");
    return [];
}
// Ziskani vsech sousedu spolu s vahami v parech (tuple)
function GetNeighbour(node) {
    var result = [];
    for (var i = 0; i < edges.edges.length; ++i) {
        if (edges.edges[i].from === node) {
            result.push([edges.edges[i].to, edges.edges[i].weight]);
        }
        if (edges.edges[i].to === node) {
            result.push([edges.edges[i].from, edges.edges[i].weight]);
        }
    }
    return result;
}
