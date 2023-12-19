use std::println;

pub fn part2(seed_ranges: &[[u64; 2]]) -> u64 {
    let seed_ranges = seed_ranges.iter().map(|array| {
        Range { start: array[0], end: array[0] + array[1] }
    }).collect();

    let seed_to_soil = "2067746708 2321931404 124423068
2774831547 3357841131 95865403
3776553292 3323317283 34523848
4167907733 3453706534 116376261
1190847573 767701596 554806188
2870696950 1975607604 173919437
1980384731 2612856575 87361977
3380570559 2987564153 335753130
3044616387 2451131599 21188806
3909556885 2167390152 154541252
3811077140 2149527041 17863111
4077167815 3804196813 90739918
2528751611 4222771775 72195521
4064098137 3894936731 13069678
4284283994 2700218552 10683302
2468832075 2472320405 59919536
3716323689 3570082795 60229603
1085396685 662250708 105450888
1030174777 1322507784 22912174
1975607604 2446354472 4777127
3828940251 2532239941 80616634
584992388 1930412346 7315040
592307428 0 437867349
1745653761 437867349 224383359
0 1345419958 584992388
2192169776 2710901854 276662299
2600947132 3630312398 173884415
1053086951 1937727386 32309734
3065805193 3908006409 314765366";

    let soil_to_fertilizer = "4148533839 3658735071 146433457
656556737 471425735 68165409
751630557 539591144 462446129
724722146 1002037273 26908411
0 24799538 226917727
4049709448 3143711443 98824391
412048729 251717265 219708470
3321838617 2645077606 72145759
3869354568 2464722726 180354880
2044900648 1380852911 42721698
226917727 1028945684 185131002
3111204505 2717223365 210634112
2129227343 1423574609 981977162
1380852911 2927857477 215853966
3393984376 2405551771 59170955
3453155331 3242535834 416199237
1596706877 3805168528 448193771
2087622346 4253362299 41604997
631757199 0 24799538";

    let fertilizer_to_water = "2679101382 2898500255 208361454
3672966601 3700867560 107718031
1764241275 4242224976 41519499
1018987051 346629037 49456831
518817716 396085868 3722249
1224466235 306138732 40490305
1068679130 947256497 122607155
1328820253 625833852 156797976
222363356 782631828 128136603
4106081288 2497628211 144953761
2887462836 1577782207 70654427
2453719866 2077230479 225381516
2177283137 3415053348 9638645
1485618229 910768431 36488066
1805760774 2833622399 32168430
482104460 0 36713256
4065926174 2480721173 16907038
118194361 213050410 93088322
1869091498 3106861709 308191639
1068443882 399808117 235248
2958117263 3926610020 315614956
1577782207 2324396806 156324367
1270677460 36713256 58142793
1264956540 586932982 5720920
1847306687 2302611995 21784811
843178107 411124038 175808944
1191286285 592653902 33179950
4251035049 4283744475 11222821
4262257870 2865790829 32709426
4082833212 2810374323 23248076
3273732219 2642581972 167792351
350499959 1069863652 131604501
3780684632 1859496740 217733739
3441524570 3808585591 118024429
3998418371 1791988937 67507803
1837929204 3691490077 9377483
1734106574 1761854236 30134701
211282683 400043365 11080673
2186921782 3424691993 266798084
522539965 1201468153 320638142
3559548999 1648436634 113417602
0 94856049 118194361";

    let water_to_light = "487890089 1253174910 48217379
1162866447 2295971038 331509140
3115016077 4085918002 209049294
3600618057 2743705059 694349239
3021490874 3712826169 26810261
2743705059 3739636430 3013944
3048301135 4019203060 66714942
1494375587 0 650888870
167398115 650888870 320491974
78943404 2207516327 88454711
3324065371 3742650374 276552686
881072381 1174877356 78297554
2690077973 2191170718 16345609
2746719003 3438054298 274771871
2145264457 1301392289 544813516
959369935 971380844 203496512
536107468 1846205805 344964913
0 2627480178 78943404";

    let light_to_temperature = "2934276762 3692860946 134937994
2222730788 3468116804 32924074
2030910720 3501040878 191820068
2876227610 3450265581 17851223
2821863146 1926340324 54364464
2894078833 3827798940 40197929
0 2499885250 950380331
3069214756 1312743837 613596487
950380331 232213448 1080530389
2302682684 2382183979 117701271
3682811243 47027822 185185626
2255654862 0 47027822
2420383955 1980704788 401479191";

    let temperature_to_humidity = "3474899002 2152529659 335631613
1227362297 2657517973 1047434675
1147289328 4214894327 80072969
3069802422 3704952648 405096580
4233143053 2090705416 61824243
3979887316 1837449679 253255737
3810530615 2488161272 169356701
1042444229 4110049228 104845099
2274796972 1042444229 718141444
2992938416 1760585673 76864006";

    let humidity_to_location = "2905941546 1669212802 106379169
3490393041 2571512629 24111360
3327134512 896350741 163258529
163044169 321738120 136537257
1794114599 1475899779 31051829
1155727752 771777629 98456450
3514504401 1890601528 199093442
3241757362 1290456090 8146812
3713597843 2595623989 446677438
2865335819 1388663285 15697510
3155553665 2166925308 86203697
26879537 567441866 52236777
421556320 59735378 198122323
4160275281 1059609270 134692015
1072793086 2488577963 82934666
2006477848 3530988938 680918581
1766297705 1528152733 5360386
1405780686 3214887893 74444149
3017788788 1533513119 135699683
977736160 4211907519 83059777
1771658091 2466121455 22456508
2687396429 870234079 26116662
79116314 547394430 20047436
2773054926 1506951608 21201125
1825166428 3289332042 181311420
2713513091 1404360795 59541835
3012320715 1213908249 5468073
3249904174 2089694970 77230338
2794256051 1219376322 71079768
1601513875 2253129005 74723447
3153488471 769712435 2065194
299581426 41300579 18434799
332437267 458275377 89119053
1273791166 2327852452 131989520
1676237322 1298602902 90060383
318016225 26879537 14421042
769712435 3470643462 60345476
2881033329 3042301427 24908217
1480224835 2459841972 6279483
99163750 257857701 63880419
1060795937 1463902630 11997149
830057911 3067209644 147678249
1254184202 1194301285 19606964
1486504318 1775591971 115009557";


    let seed_to_soil = parse_map(seed_to_soil);
    let soil_to_fertilizer = parse_map(soil_to_fertilizer);
    let fertilizer_to_water = parse_map(fertilizer_to_water);
    let water_to_light = parse_map(water_to_light);
    let light_to_temperature = parse_map(light_to_temperature);
    let temperature_to_humidity = parse_map(temperature_to_humidity);
    let humidity_to_location = parse_map(humidity_to_location);

    let soils = transform(&seed_ranges, &seed_to_soil);
    let fertilizers = transform(&soils, &soil_to_fertilizer);
    let waters = transform(&fertilizers, &fertilizer_to_water);
    let lights = transform(&waters, &water_to_light);
    let temperatures = transform(&lights, &light_to_temperature);
    let humidities = transform(&temperatures, &temperature_to_humidity);
    let mut locations = transform(&humidities, &humidity_to_location);
    locations.sort_by_key(|l| l.start);

    println!("{:?}", &locations);

    locations.iter().min_by_key(|l| l.start).unwrap().start
}

#[derive(Debug)]
struct Map {
    src: Vec<Range>,
    dest: Vec<Range>,
}

type Range = std::ops::Range<u64>;

fn parse_line(line: &str) -> (Range, Range) {
    let mut line = line.split_whitespace();
    let dest_start = line.next().unwrap().parse::<u64>().unwrap();
    let src_start = line.next().unwrap().parse::<u64>().unwrap();
    let len = line.next().unwrap().parse::<u64>().unwrap();

    // (destination_range, source_range)
    (
        Range {
        start: dest_start,
        end: dest_start + len
        },
        Range {
            start: src_start,
            end: src_start + len
        }
    )
}

fn parse_map(map: &str) -> Map {
    let mut src = vec![];
    let mut dest = vec![];

    map.lines().for_each(|line| {
        let (d, s) = parse_line(line);
        dest.push(d); src.push(s);
    });

    Map { src, dest }
}

fn transform(input_ranges: &mut Vec<Range>, map: &Map) -> Vec<Range> {
    let mut output_ranges = Vec::with_capacity(input_ranges.len());

    //println!("\tinput_ranges: {:?}", &input_ranges);
    //println!("\tsource ranges: {:?}", &map.src);
    //println!("\tdestinations: {:?}", &map.dest);

    input_ranges.iter().for_each(|input_range| {
        let mut was_processed = false;

        //println!("\tinput_range = {}-{}", input_range.start, input_range.end);

        for (i, src_range) in map.src.iter().enumerate() {
            if was_processed {
                break;
            }

            if input_range.start >= src_range.start && input_range.end <= src_range.end {
                //println!("\t\tFirst if");
                //println!("\t\tsrc_range: {:?}", &src_range);
                //println!("\t\tdst_range: {:?}", &map.dest[i]);
                was_processed = true;

                output_ranges.push(
                    Range {
                        start: map.dest[i].start + (input_range.start - src_range.start),
                        end: map.dest[i].end - (src_range.end - input_range.end),
                    }
                );
                //println!("\t\toutput_ranges: {:?}", &output_ranges);
            } else if input_range.start < src_range.start && input_range.end >= src_range.start {
                //println!("\t\tSecond if");
                //println!("\t\tsrc_range: {:?}", &src_range);
                // ---  (input)
                //  --- (src)
                was_processed = true;

                output_ranges.push(
                    Range {
                        start: input_range.start,
                        end: src_range.start,
                    }
                );

                if input_range.end > src_range.end {
                    output_ranges.push(
                        Range {
                            start: map.dest[i].start,
                            end: map.dest[i].end,
                        }
                    );
                } else {
                    output_ranges.push(
                        Range {
                            start: map.dest[i].start,
                            end: map.dest[i].end - (src_range.end - input_range.end),
                        }
                    );
                }
                //println!("\t\tdst_range: {:?}", &map.dest[i]);
                //println!("\t\toutput_ranges: {:?}", &output_ranges);
            } else if input_range.start <= src_range.end && input_range.end > src_range.end {
                //println!("\t\tThird if");
                //println!("\t\tsrc_range: {:?}", &src_range);
                //   --- (input)
                //  ---  (src)
                was_processed = true;

                output_ranges.push(
                    Range {
                        start: map.dest[i].start + (input_range.start - src_range.start),
                        end: map.dest[i].end,
                    }
                );
                output_ranges.push(
                    Range {
                        start:  src_range.end,
                        end: input_range.end,
                    }
                );
                //println!("\t\toutput_ranges: {:?}", &output_ranges);
            }
        }

        if !was_processed {
            output_ranges.push(
                input_range.clone()
            );
        }

        //println!("\t\toutput_ranges: {:?}", &output_ranges);
    });

    output_ranges
}

#[test]
fn test_part2() {
    let ranges = [
        [79, 14],
        [55, 13],
    ];

    let mut ranges = ranges.iter().map(|array| {
        Range { start: array[0], end: array[0] + array[1] }
    }).collect();

    dbg!(&ranges);

    let seed_to_soil = "50 98 2
52 50 48";
    let seed_to_soil = parse_map(seed_to_soil);
    transform(&mut ranges, &seed_to_soil);

    dbg!(&soils);
    
    let soil_to_fertilizer = "0 15 37
37 52 2
39 0 15";
    let soil_to_fertilizer = parse_map(soil_to_fertilizer);
    let fertilizers = transform(&soils, &soil_to_fertilizer);

    dbg!(&soil_to_fertilizer);
    dbg!(&fertilizers);

    let fertilizer_to_water = "49 53 8
0 11 42
42 0 7
57 7 4";
    let fertilizer_to_water = parse_map(fertilizer_to_water);
    let waters = transform(&fertilizers, &fertilizer_to_water);

    dbg!(&fertilizer_to_water);
    dbg!(&waters);

    let water_to_light = "88 18 7
18 25 70";
    let water_to_light = parse_map(water_to_light);
    let lights = transform(&waters, &water_to_light);

    dbg!(&water_to_light);
    dbg!(&lights);

    let light_to_temperature = "45 77 23
81 45 19
68 64 13";
    let light_to_temperature = parse_map(light_to_temperature);
    let temperatures = transform(&lights, &light_to_temperature);

    dbg!(&light_to_temperature);
    dbg!(&temperatures);

    let temperature_to_humidity = "0 69 1
1 0 69";
    let temperature_to_humidity = parse_map(temperature_to_humidity);
    let humidities = transform(&temperatures, &temperature_to_humidity);

    dbg!(&temperature_to_humidity);
    dbg!(&humidities); // 9..1

    let humidity_to_location = "60 56 37
56 93 4";
    let humidity_to_location = parse_map(humidity_to_location);
    let locations = transform(&humidities, &humidity_to_location);

    dbg!(&humidity_to_location);
    dbg!(&locations);

    assert_eq!(46, locations.iter().min_by_key(|l| l.start).unwrap().start);
}
