import "./global.js"
import "./db.js"
import {
    CalculateAllStats,
    CalculateEnemyStats,
    CalculateBattle,
    buildEquipment,
    buildCard,
    GetTestCase
} from "./calc.js"
import fs from "fs";
import path from "path";

// const command = "console";
//
// switch (command) {
//     case "convert":
//         break;
//     case "console":
//         let formData = JSON.parse(fs.readFileSync('./test-data.json', 'utf-8'))
//         let testCase= GetTestCase(formData);
//         break;
// }

let count = 0;
let start = Date.now();
let testcases = [];
for (let n = 1; n < JobName.length; n++) {
    let formData = {};
    formData.A_JobLV = 1;
    formData.A_JOB = n;
    formData.A_BaseLV = 95;
    formData.A_STR = 50;
    formData.A_AGI = 50;
    formData.A_VIT = 50;
    formData.A_INT = 50;
    formData.A_DEX = 50;
    formData.A_LUK = 50;
    formData.A_Arrow = 0;

    formData.A_acces1 = "326";
    formData.A_acces2 = "326";
    formData.A_left = "305";
    formData.A_body = "279";
    formData.A_head1 = "142";
    formData.A_head2 = "243";
    formData.A_head3 = "268";
    formData.A_shoes = "317";
    formData.A_shoulder = "311";
    formData.A_weapon1 = "0";
    formData.A_WeaponType = "0";

    formData.B_Enemy = 272;
    for (let i = 0; i < JobSkillPassOBJ[n].length; i++) {
        if (JobSkillPassOBJ[n][i] != 999) {
            formData["A_PASSIVE_SKILL" + i] = SkillOBJ[JobSkillPassOBJ[n][i]][1];
        }
    }
    for (let i = 0; i < JobSkillActiveOBJ[n].length && JobSkillActiveOBJ[n][i] != 999; i++) {
        formData.A_ActiveSkill = JobSkillActiveOBJ[n][i];
        switch (formData.A_ActiveSkill) {
            case 326:
            case 66:
                formData.SkillSubNum = 8000;
                break;
            case 112:
            case 113:
                formData.SkillSubNum = 1;
                break;
            case 131:
                formData.SkillSubNum = 5;
                break;
            case 88:
                formData.SkillSubNum = 5;
                break;
            case 197:
            case 405:
                formData.SkillSubNum = 1000;
                break;
            case 394:
            case 395:
                formData.SkillSubNum = 1;
                break;
        }
        // *****************for each cards********************
        // for (let c = 0; c < 121; c++) {
        //     formData.A_acces1_card = global.CardSortOBJ[7][c] && global.CardSortOBJ[7][c] !== "NULL" ? cardOBJ[global.CardSortOBJ[7][c]][0] : cardOBJ[global.CardSortOBJ[7][0]][0]
        //     formData.A_acces2_card = global.CardSortOBJ[7][c] && global.CardSortOBJ[7][c] !== "NULL" ? cardOBJ[global.CardSortOBJ[7][c]][0] : cardOBJ[global.CardSortOBJ[7][0]][0]
        //     formData.A_left_card = global.CardSortOBJ[3][c] && global.CardSortOBJ[3][c] !== "NULL" ? cardOBJ[global.CardSortOBJ[3][c]][0] : cardOBJ[global.CardSortOBJ[3][0]][0]
        //     formData.A_body_card = global.CardSortOBJ[4][c] && global.CardSortOBJ[4][c] !== "NULL" ? cardOBJ[global.CardSortOBJ[4][c]][0] : cardOBJ[global.CardSortOBJ[4][0]][0]
        //     formData.A_head1_card = global.CardSortOBJ[2][c] && global.CardSortOBJ[2][c] !== "NULL" ? cardOBJ[global.CardSortOBJ[2][c]][0] : cardOBJ[global.CardSortOBJ[2][0]][0]
        //     formData.A_head2_card = global.CardSortOBJ[2][c] && global.CardSortOBJ[2][c] !== "NULL" ? cardOBJ[global.CardSortOBJ[2][c]][0] : cardOBJ[global.CardSortOBJ[2][0]][0]
        //     formData.A_shoes_card = global.CardSortOBJ[6][c] && global.CardSortOBJ[6][c] !== "NULL" ? cardOBJ[global.CardSortOBJ[6][c]][0] : cardOBJ[global.CardSortOBJ[6][0]][0]
        //     formData.A_shoulder_card = global.CardSortOBJ[5][c] && global.CardSortOBJ[5][c] !== "NULL" ? cardOBJ[global.CardSortOBJ[5][c]][0] : cardOBJ[global.CardSortOBJ[5][0]][0]
        //     formData.A_weapon1_card1 = global.CardSortOBJ[1][c] && global.CardSortOBJ[1][c] !== "NULL" ? cardOBJ[global.CardSortOBJ[0][c]][0] : cardOBJ[global.CardSortOBJ[0][0]][0]
        //     formData.A_weapon1_card2 = global.CardSortOBJ[1][c] && global.CardSortOBJ[1][c] !== "NULL" ? cardOBJ[global.CardSortOBJ[1][c]][0] : cardOBJ[global.CardSortOBJ[1][0]][0]
        //     formData.A_weapon1_card3 = global.CardSortOBJ[1][c] && global.CardSortOBJ[1][c] !== "NULL" ? cardOBJ[global.CardSortOBJ[1][c]][0] : cardOBJ[global.CardSortOBJ[1][0]][0]
        //     formData.A_weapon1_card4 = global.CardSortOBJ[1][c] && global.CardSortOBJ[1][c] !== "NULL" ? cardOBJ[global.CardSortOBJ[1][c]][0] : cardOBJ[global.CardSortOBJ[1][0]][0]
        //     formData.A_ActiveSkillLV = SkillOBJ[JobSkillActiveOBJ[n][i]][1];
        //     console.log("generate test case for job", n, "skill", i, "(", SkillOBJ[JobSkillActiveOBJ[n][i]][2], ")");
        //     let testCase = GetTestCase(formData);
        //     count += 1;
        // }

        // *****************for each monster********************
        // for (let m = 0; m < MonsterOBJ.length; m++) {
        //     formData.B_Enemy = m;
        //     formData.A_ActiveSkillLV = SkillOBJ[JobSkillActiveOBJ[n][i]][1];
        //     console.log("generate test case for job", n, "skill", i, "(", SkillOBJ[JobSkillActiveOBJ[n][i]][2], ")", "target", m);
        //     let testCase = GetTestCase(formData);
        //     count += 1;
        // }


        // *****************for each weapon type********************
        // let weaponTypes = [];
        // for (let i = 0; i <= 21; i++) {
        //     if (JobASPD[n][i] != 0) {
        //         weaponTypes.push(i);
        //     }
        // }
        // let weapons = [];
        // for(let weaponType of weaponTypes) {
        //     for (let i = 0; i < global.ItemOBJ.length; i++) {
        //         if (global.ItemOBJ[i][1] === weaponType) {
        //             weapons.push({index: i, type: weaponType});
        //             break;
        //         }
        //     }
        // }
        // for(let weapon of weapons) {
        //
        // }

        formData.A_ActiveSkillLV = SkillOBJ[JobSkillActiveOBJ[n][i]][1];
        console.log("generate test case for job", n, "skill", i, "(", SkillOBJ[JobSkillActiveOBJ[n][i]][2], ")");
        let testCase = GetTestCase(formData);
        count += 1;
        // console.log(testCase)
        testcases.push(testCase)
    }

}
console.log("Generated", count, "test cases, in", (Date.now() - start) + "ms")
fs.writeFileSync(path.join(process.cwd(), "..\\..\\server\\src\\tests\\common\\fixtures\\data\\battle-all-skills-no-stuff.json"), JSON.stringify(testcases))