const graf = [
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

const edges = {
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
  const canvas = <HTMLCanvasElement>document.getElementById("canvas");

  if (canvas?.getContext) {
    const ctx = canvas?.getContext("2d");
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
function DrawGraph(ctx: CanvasRenderingContext2D) {
  // Zakladni nastaveni vizualu grafu s designem textu
  ctx.beginPath();
  ctx.strokeStyle = "#000000";
  ctx.font = "bold 16px Arial";
  ctx.textAlign = "center";
  ctx.textBaseline = "middle";

  for (let i = 0; i < edges.edges.length; ++i) {
    ctx.fillStyle = "blue";
    // Nastaveni zacatecniho a konecneho cile
    const originNode = graf.find((item) => edges.edges[i].from === item.label);
    const destNode = graf.find((item) => edges.edges[i].to === item.label);

    if (!originNode || !destNode) {
      console.error("Zvolený uzel neexistuje!");
      return;
    }

    // Vykresleni zacatku a jeho bodu s pismenem
    ctx.moveTo(originNode.x, originNode?.y);
    ctx.fillText(originNode.label, originNode.x, originNode.y + 20);
    ctx.fillRect(originNode.x - 7, originNode.y + 7, 10, -10);

    // Vykresleni konce a jeho bodu s pismenem
    ctx.lineTo(destNode.x, destNode.y);
    ctx.fillText(destNode.label, destNode.x, destNode.y + 20);
    ctx.fillRect(destNode.x - 7, destNode.y + 7, 10, -10);

    // Nakresleni cary od zaccatku po konec
    ctx.stroke();

    // Nakresleni vahy
    const wX = (originNode.x + destNode.x) / 2;
    const wY = (originNode.y + destNode.y) / 2;
    ctx.fillStyle = "red";
    for (let i = 0; i < edges.edges.length; ++i) {
      if (
        edges.edges[i].from === originNode?.label &&
        edges.edges[i].to === destNode?.label
      ) {
        ctx.fillText(edges.edges[i].weight.toString(), wX, wY);
      }
    }
  }
}

// Zobrazeni nejrychlejsi cesty (od zacatku po konec)
function DisplayPath(ctx: CanvasRenderingContext2D) {
  // Nacteni dat z form na strance
  const startElement = document.getElementById(
    "start"
  ) as HTMLInputElement | null;
  const endElement = document.getElementById("end") as HTMLInputElement | null;

  // Konverze na String
  const start: string | undefined = startElement?.value;
  const end: string | undefined = endElement?.value;

  // Detekce chyby, jestli hodnoty neexistuje, doslo k chybe
  if (!start || !end) {
    console.error("Neexistující uzel!");
    return;
  }

  // Jestli je zadana hodnota delsi nez jeden znak nebo neni pismeno, tak detekujeme chybu
  if (
    start.length != 1 ||
    end.length != 1 ||
    !isNaN(Number(start)) ||
    !isNaN(Number(end))
  ) {
    console.error("Nesprávna hodnota, zadejte jedno písmeno");
    return;
  }

  // Vypocteni a nalezeni vzdalenosti
  const path = CaculateDjikstra(start, end);

  // Prekresleni linie
  ctx.beginPath();
  for (let i = 0; i < path.length; ++i) {
    ctx.strokeStyle = "#00ff55";

    // Nastaveni zacatecniho a konecneho cile
    const originNode = graf.find((item) => path[i] === item.label);
    const destNode = graf.find((item) => path[i + 1] === item.label);

    if (!originNode || !destNode) {
      console.error("Body neexistují");
      return;
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
  }
}

function CaculateDjikstra(start: string, end: string): string[] {
  const queue: string[] = [];
  const previous: any = {};
  const distances: any = {};

  for (const node in edges.vertices) {
    distances[edges.vertices[node]] = Infinity;
    previous[edges.vertices[node]] = null;
    queue.push(edges.vertices[node]);
  }
  distances[start] = 0;

  while (queue.length > 0) {
    // Na ziskani nejkratsi vzdalenosti muzeme udelat sort
    queue.sort((a, b) => distances[a] - distances[b]);

    const currentNode = queue.shift();
    // Pokud najdeme cil, funkce vraci cestu
    if (currentNode == end) {
      const path = [];
      let temp = currentNode;

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

    const neighbors = GetNeighbour(currentNode);

    // Prohledani sousedu
    for (let i = 0; i < neighbors.length; ++i) {
      const alt = distances[currentNode] + neighbors[i][1];
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
function GetNeighbour(node: string): [string, number][] {
  let result: [string, number][] = [];

  for (let i = 0; i < edges.edges.length; ++i) {
    if (edges.edges[i].from === node) {
      result.push([edges.edges[i].to, edges.edges[i].weight]);
    }
    if (edges.edges[i].to === node) {
      result.push([edges.edges[i].from, edges.edges[i].weight]);
    }
  }

  return result;
}
