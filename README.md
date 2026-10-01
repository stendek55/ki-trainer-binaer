---
## ............*in bearbeitung*..............
---

# ki-trainer-binaer
**zum erkennen handgeschriebener zahlen soll eigenes ki modell trainiert werden auf grundlage von bitoperationen?!?**
# Challenge: Bit-KI für Handschriften - Built from Scratch in Rust

## Worum geht es überhaupt?
Ich will wissen, ob es möglich ist, ein eigenes KI-Modell zur Erkennung von handschriftlichen Zahlen zu bauen, das ohne den typischen Mathe-Ballast auskommt. Normale KIs fressen Unmengen an Strom und Rechenleistung, weil sie ständig mit fetten Kommazahlen (Float32) jonglieren. Mein Ziel: Eine KI, die ausschließlich mit Low-Level-Bitoperationen (AND, OR, XOR, XNOR, NOT)??? auf der CPU arbeitet. Superschnell, superleicht und perfekt für winzige Hardware.

Da ich mir in vergangenen Projekten die Grundlagen von Rust erarbeitet habe, ziehe ich das Ganze komplett in Rust hoch. Rust ist durch seine Performance und Typsicherheit perfekt für dieses hardwarenahe Bit-Gebastel.

## Meine wichtigste Regel: Keine vorgefertigten Wege!
Ich will nicht einfach bestehende Papers (wie BNNs oder Tsetlin-Maschinen) eins zu eins kopieren oder mich von fertigen Frameworks in eine bestimmte Richtung lenken lassen. Ich möchte das Rad bewusst ein bisschen selbst neu erfinden, eigene Logik-Ansätze testen und schauen, wie weit ich mit meinen eigenen Ideen komme. 

Ich werde KI-Tools als Beschleuniger einsetzen, um Code-Strukturen zu optimieren oder Denkblockaden zu lösen - das eigentliche Architektur-Design und die Algorithmen sollen aber meinen eigenen Ideen entspringen.

## Was ich mir vorgenommen habe (Meine Meilensteine)

### 1. Der eigene Datengenerator
Ich verlasse mich nicht nur auf fertige Datensätze wie MNIST. Ich baue mir in Rust einen eigenen Datengenerator, der:
* Handgeschriebene Bitmaps (Binärbilder) von Ziffern erzeugt.
* Eigene "Bit-Verschiebe-Funktionen" nutzt (Bit-Shifting für Bewegung, Bit-Invertierung für Bildrauschen), um das Modell später richtig auf die Probe zu stellen.

### 2. Das Bit-Modell (Die eigentliche KI)
* Ich designe ein System in Rust, das Bilder einliest und die Klassifizierung rein über logische Verknüpfungen von Bits regelt.
* Ich will herausfinden, wie gut ein rein logisches Regelwerk performen kann.
* Kann ich alle Werte im u8 Bereich halten?

### 3. Der Härtetest
* Am Ende will ich schwarz auf weiß sehen: Funktioniert mein eigener Ansatz? 
* Wie schlägt sich meine Rust-Bit-KI auf meinen eigenen Daten und wie performant ist sie?

## Tech Stack und Skills
* Programmiersprache: Rust (Bit-Manipulation, hardwarenaher Code)
* Konzept: Edge-AI (Ressourceneffiziente Algorithmen ohne Ballast)
* Data Engineering: Eigene Datengenerierung und Bildbearbeitung auf Bitebene
* Arbeitsweise: KI-unterstützte Entwicklung (KI als Programmier-Copilot)

## Die Kernfrage, die ich mir selbst beantworte:
Kann man ein funktionsfähiges KI-Modell nur mit logischem Denken, ein paar Bits und feinstem Rust-Code bauen, ohne den Mainstream-Pfaden zu folgen?

## Anwendung zum Zeichnen und Speichern der Zahlen
![zahlenzeichner](eins.png)

---
---
## ............*aktueller zwischenbericht*..............
***auswertung durch ki***
# Projektübersicht: Maschinelles Lernen auf Binärebene in Rust

Dieses Projekt implementiert eine vollständige Pipeline zur Erzeugung, Visualisierung und Klassifizierung von handschriftlichen Mustern auf reiner Binärebene (16x16 Pixel) ohne externe Abhängigkeiten für die mathematischen Kernoperationen.

---

## 1. Daten-Augmentation (`tools/erweitern.rs`)
Dieses Skript dient der künstlichen Vergrößerung und Modifikation des bestehenden Datensatzes, um die Robustheit des späteren Modells zu verbessern.

* **Datenimport:** Es liest bestehende Muster aus einer CSV-Datei ein und validiert, ob die Zeilen exakt dem Format von 1 Label plus 256 Pixelwerten (16x16 Matrix) entsprechen.
* **Geometrische Transformationen:** Das Skript enthält Funktionen für Dilatation (Aufdickung von Linien nach links-oben, rechts-unten oder achsenspezifisch gesplittet) sowie für einen zentrumsgesicherten Zoom nach außen.
* **Translation (Verschiebung):** Der aktive Code verschiebt die geladenen Pixelmuster testweise um eine definierte Pixelanzahl in vier Richtungen (Oben, Unten, Links, Rechts). Ein Randschutz stellt sicher, dass Muster verworfen werden, sobald relevante Bildinformationen über die Gittergrenzen hinaus abgeschnitten würden.
* **Datenexport:** Die generierten synthetischen Muster werden zusammen mit den Originalen sequentiell in eine neue CSV-Datei geschrieben.

---

## 2. Datensatz-Visualisierung (`tools/sehen.rs`)
Dieses Skript stellt eine grafische Benutzeroberfläche bereit, um die in der CSV-Datei gespeicherten Binärmuster visuell zu prüfen.

* **Nearest-Neighbor-Upsampling:** Da die Quellmatrizen mit 16x16 Pixeln zu klein für eine menschliche Analyse sind, projiziert das Skript die Koordinaten mittels Ganzzahl-Division auf ein skaliertes Fenster von 368x368 Pixeln hoch.
* **Interaktive Navigation:** Über die FFI-Schnittstelle der `minifb`-Bibliothek wird ein OS-Fenster geöffnet. Der Nutzer kann mit den Pfeiltasten (Links/Rechts) nicht-blockierend durch den gesamten Datensatz blättern.
* **Metadaten-Anzeige:** Der Fenstertitel wird dynamisch aktualisiert und zeigt fortlaufend den aktuellen Index des Musters sowie das zugehörige numerische Klassen-Label an.

---

## 3. Grafischer Daten-Generator (`tools/zeichner.rs`)
Dieses Skript erlaubt es, eigene Trainingsdaten interaktiv per Maus zu zeichnen und direkt im passenden CSV-Format abzuspeichern.

* **Zeichenfläche:** Es initialisiert ein flaches 1D-Array im Arbeitsspeicher, welches das logische 16x16-Gitter repräsentiert. Die linke Maustaste setzt Pixel auf den Zustand 1 (aktiv/schwarz), die rechte Maustaste radiert Pixel zurück auf den Zustand 0 (inaktiv/weiß).
* **Gitter-Rendering:** Zur Orientierung wird während des Zeichenvorgangs ein hellgraues Linienraster in den Framebuffer berechnet.
* **Klassifizierung und Datei-I/O:** Durch Drücken der Tasten `0` oder `1` wird das gezeichnete Muster mit der entsprechenden Klasse versehen. Das Skript opens die Zieldatei im Append-Modus und hängt das Label gefolgt von den kommagetrennten 256 Binärwerten als neue Zeile an. Anschließend wird das Zeichenfeld automatisch geleert.

---

## 4. Binäres Neuronales Netzwerk (`src/lib.rs`)
Dieses Skript implementiert die Kernlogik des Klassifikationsmodells, das vollständig auf Bitoperationen und hardwarenahen Berechnungen aufbaut.

* **BitByte-Wrapper:** Es definiert eine eigene Datenstruktur für Byte-Werte, die bitweise Operationen (AND, OR, XOR, NOT, Bit-Shifts) und gezieltes Bit-Toggling über Punktnotation (Method Chaining) ermöglicht.
* **Netzwerk-Architektur:** Das Modell nutzt ein Feedforward-Netzwerk mit drei verdeckten Schichten (64, 32 und 16 binäre Knoten) sowie einer Ausgabeschicht mit 3 Knoten für die Zustände NULL, EINS und ANDERE. Jeder Knoten (`BinaryNode`) besitzt eine Schablone aus gelernten Bit-Mustern (Gewichten) und einen individuellen Schwellenwert (Threshold).
* **Forward Pass via Bit-Matches:** Die Klassifizierung erfolgt ohne Fließkomma-Arithmetik. Das System berechnet die Übereinstimmungen zwischen Mustern und Knotengewichten über eine XNOR-Logik mit anschließendem Hardware-Popcount (`count_ones`). Erreicht die Anzahl der übereinstimmenden Bits den Schwellenwert, feuert das Bit für die nächste Schicht.
* **Evaluierung und Evolution (Mutation):** Das Netzwerk bietet Methoden zur Bewertung der Gesamt-Fitness anhand eines Testdatensatzes. Da kein klassisches Backpropagation genutzt wird, simuliert eine Mutationsfunktion evolutionäre Anpassungen, indem sie Bytes und Schwellenwerte basierend auf einer definierten Rate gezielt per Zufall verändert. Ein Winner-Takes-All-Verfahren mit eingebauter Konfidenz-Klaue (Mindestvorsprung von 2 Punkten) sichert das finale Klassifikationsergebnis ab.

