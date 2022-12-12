(ns aoc22.day7
  (:require
   [clojure.java.io :as io]
   [clojure.string :as str]
   [clojure.test :refer [deftest is]]
   [clojure.walk :as walk]))

(defn dir [s] {(str/join (drop 4 s)) {}})

(defn file [s]
  (let [[size name] (str/split s #" ")]
    {name (parse-long size)}))

(defn cd [s] (str/join (drop 5 s)))

(defn ls->map [ls]
  (->> ls
       (map #(if (str/starts-with? % "dir ") (dir %) (file %)))
       (apply merge)))

(defn file-line? [s] (not (str/starts-with? s "$")))

(defn parse [s]
  ; rest so we skip the "$ cd /" at the top
  (loop [lines (rest (str/split-lines s))
         path  ["/"]
         tree  {"/" {}}]
    (if-let [[line & lines] (seq lines)]
      (cond
        (= line "$ ls") (let [[ls lines] (split-with file-line? lines)
                              elements (ls->map ls)
                              tree     (update-in tree path merge elements)]
                          (recur lines path tree))
        (= line "$ cd ..") (recur lines (vec (drop-last path)) tree)
        (str/starts-with? line "$ cd ") (recur lines (conj path (cd line)) tree))
      tree)))

(defn sum-dir [input]
  (walk/postwalk
   (fn [node] (if (map? node)
                (reduce + (vals node))
                node))
   input))

(defn dir-sizes [input]
  (loop [tree  input
         path  ["/"]
         sizes {}]
    (if-let [[next _] (first (filter #(map? (val %)) (get-in tree path)))]
      (recur tree (conj path next) sizes)
      (let [size (reduce + (vals (get-in tree path)))]
        (if (= path ["/"])
          (assoc sizes path size)
          (recur (assoc-in tree path size)
                 (vec (drop-last path))
                 (assoc sizes path size)))))))

(defn part1 [input]
  (let [sizes (dir-sizes input)]
    (->> sizes
         vals
         (filter #(< % 100000))
         (reduce +))))

(defn part2 [input]
  (let [sizes  (dir-sizes input)
        root   (get sizes ["/"])
        target (- 30000000 (- 70000000 root))]
    (->> sizes
         vals
         (filter #(>= % target))
         (apply min))))

(deftest tests
  (let [sample "$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k"
        input  (parse sample)]
    (is (= input {"/" {"a"
                       {"e"
                        {"i" 584}
                        "f"     29116
                        "g"     2557
                        "h.lst" 62596}
                       "b.txt" 14848514
                       "c.dat" 8504156
                       "d"     {"j"     4060174
                                "d.log" 8033020
                                "d.ext" 5626152
                                "k"     7214296}}}) "parse")
    (is (= (part1 input) 95437) "part1")
    (is (= (part2 input) 24933642) "part2")))

(defn -main []
  (let [file  (slurp (io/resource "7.txt"))
        input (parse file)]
    (println "Part 1:" (part1 input))
    (println "Part 2:" (part2 input))))
