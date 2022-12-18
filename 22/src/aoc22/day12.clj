(ns aoc22.day12
  (:require
   [clojure.data.priority-map :refer [priority-map]]
   [clojure.java.io :as io]
   [clojure.string :as str]
   [clojure.test :refer [deftest is]]))

(defn get-coord [coll [x y]]
  (get (get coll y) x))

(defn replace-coord [coll [x y] v]
  (assoc-in coll [y x] v))

(defn grid-search [grid c]
  (for [[y row] (zipmap (range) grid)
        [x v] (zipmap (range) row)
        :when (= v c)]
    [x y]))

(defn parse [s]
  (let [grid  (mapv #(vec (char-array %)) (str/split-lines s))
        start (first (grid-search grid \S))
        end   (first (grid-search grid \E))]
    {:start start
     :end   end
     :grid  (as-> grid g
                  (replace-coord g start \a)
                  (replace-coord g end \z))}))

(defn could-climb-to [grid [x y]]
  (let [h      (get-coord grid [x y])
        coords [[(inc x) y]
                [(dec x) y]
                [x (inc y)]
                [x (dec y)]]]
    (filter
     (fn [[x y]]
       (if-let [h' (get-coord grid [x y])]
         (>= (inc (int h)) (int h'))))
     coords)))

(defn could-climb-from [grid [x y]]
  (let [h      (get-coord grid [x y])
        coords [[(inc x) y]
                [(dec x) y]
                [x (inc y)]
                [x (dec y)]]]
    (filter
     (fn [[x y]]
       (if-let [h' (get-coord grid [x y])]
         (>= (inc (int h')) (int h))))
     coords)))

(defn run [{grid :grid, start :start} neighbours-f end?]
  (loop [xy-current start
         d-current  0
         visited    #{}
         distances  (priority-map start 0)]
    (if (end? xy-current)
      d-current
      (let [tentative           (neighbours-f grid xy-current)
            d                   (inc d-current)
            tentative-distances (map #(if-let [d' (get distances %)] (min d d') d) tentative)
            distances           (merge distances (zipmap tentative tentative-distances))
            [xy-next d-next]    (peek distances)]
        (recur xy-next d-next (conj visited xy-current) (pop distances))))))

(defn part1 [input] (run input could-climb-to #(= (:end input) %)))

(defn part2-slow [{grid :grid, end :end}]
  (let [starts (grid-search grid \a)]
    (reduce min (pmap #(part1 {:grid grid, :end end, :start %}) starts))))

(defn part2 [{grid :grid, end :end}]
  (let [starts (set (grid-search grid \a))]
    (run {:grid grid, :start end} could-climb-from #(contains? starts %))))

(deftest tests
  (let [sample "Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi"
        input  (parse sample)]
    (is (= (:start input) [0 0]) "parse sample :start")
    (is (= (:end input) [5 2]) "parse sample :end")
    (is (= (first (:grid input)) [\a \a \b \q \p \o \n \m]) "parse sample :grid y=0")
    (is (= (nth (:grid input) 2) [\a \c \c \s \z \z \x \k]) "parse sample :grid y=2")
    (is (= (could-climb-to (:grid input) [0 0]) [[1 0] [0 1]]))
    (is (= (grid-search (:grid input) \a) [[0 0] [1 0] [0 1] [0 2] [0 3] [0 4]]) "grid-search")
    (is (= (part1 input) 31) "part1")
    (is (= (part2 input) 29) "part2")))

(defn -main []
  (let [file  (slurp (io/resource "12.txt"))
        input (parse file)]
    (println "Part 1:" (part1 input))
    (println "Part 2:" (part2 input))))
