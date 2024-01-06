
import type { CustomThemeConfig } from '@skeletonlabs/tw-plugin';

export const myCustomTheme: CustomThemeConfig = {
    name: 'my-custom-theme',
    properties: {
		// =~= Theme Properties =~=
		"--theme-font-family-base": `system-ui`,
		"--theme-font-family-heading": `system-ui`,
		"--theme-font-color-base": "0 0 0",
		"--theme-font-color-dark": "255 255 255",
		"--theme-rounded-base": "9999px",
		"--theme-rounded-container": "8px",
		"--theme-border-base": "1px",
		// =~= Theme On-X Colors =~=
		"--on-primary": "255 255 255",
		"--on-secondary": "0 0 0",
		"--on-tertiary": "255 255 255",
		"--on-success": "0 0 0",
		"--on-warning": "255 255 255",
		"--on-error": "0 0 0",
		"--on-surface": "0 0 0",
		// =~= Theme Colors  =~=
		// primary | #211216 
		"--color-primary-50": "222 219 220", // #dedbdc
		"--color-primary-100": "211 208 208", // #d3d0d0
		"--color-primary-200": "200 196 197", // #c8c4c5
		"--color-primary-300": "166 160 162", // #a6a0a2
		"--color-primary-400": "100 89 92", // #64595c
		"--color-primary-500": "33 18 22", // #211216
		"--color-primary-600": "30 16 20", // #1e1014
		"--color-primary-700": "25 14 17", // #190e11
		"--color-primary-800": "20 11 13", // #140b0d
		"--color-primary-900": "16 9 11", // #10090b
		// secondary | #2bcffc 
		"--color-secondary-50": "223 248 255", // #dff8ff
		"--color-secondary-100": "213 245 254", // #d5f5fe
		"--color-secondary-200": "202 243 254", // #caf3fe
		"--color-secondary-300": "170 236 254", // #aaecfe
		"--color-secondary-400": "107 221 253", // #6bddfd
		"--color-secondary-500": "43 207 252", // #2bcffc
		"--color-secondary-600": "39 186 227", // #27bae3
		"--color-secondary-700": "32 155 189", // #209bbd
		"--color-secondary-800": "26 124 151", // #1a7c97
		"--color-secondary-900": "21 101 123", // #15657b
		// tertiary | #411570 
		"--color-tertiary-50": "227 220 234", // #e3dcea
		"--color-tertiary-100": "217 208 226", // #d9d0e2
		"--color-tertiary-200": "208 197 219", // #d0c5db
		"--color-tertiary-300": "179 161 198", // #b3a1c6
		"--color-tertiary-400": "122 91 155", // #7a5b9b
		"--color-tertiary-500": "65 21 112", // #411570
		"--color-tertiary-600": "59 19 101", // #3b1365
		"--color-tertiary-700": "49 16 84", // #311054
		"--color-tertiary-800": "39 13 67", // #270d43
		"--color-tertiary-900": "32 10 55", // #200a37
		// success | #21c35d 
		"--color-success-50": "222 246 231", // #def6e7
		"--color-success-100": "211 243 223", // #d3f3df
		"--color-success-200": "200 240 215", // #c8f0d7
		"--color-success-300": "166 231 190", // #a6e7be
		"--color-success-400": "100 213 142", // #64d58e
		"--color-success-500": "33 195 93", // #21c35d
		"--color-success-600": "30 176 84", // #1eb054
		"--color-success-700": "25 146 70", // #199246
		"--color-success-800": "20 117 56", // #147538
		"--color-success-900": "16 96 46", // #10602e
		// warning | #0b1ee6 
		"--color-warning-50": "218 221 251", // #daddfb
		"--color-warning-100": "206 210 250", // #ced2fa
		"--color-warning-200": "194 199 249", // #c2c7f9
		"--color-warning-300": "157 165 245", // #9da5f5
		"--color-warning-400": "84 98 238", // #5462ee
		"--color-warning-500": "11 30 230", // #0b1ee6
		"--color-warning-600": "10 27 207", // #0a1bcf
		"--color-warning-700": "8 23 173", // #0817ad
		"--color-warning-800": "7 18 138", // #07128a
		"--color-warning-900": "5 15 113", // #050f71
		// error | #ccb4ae 
		"--color-error-50": "247 244 243", // #f7f4f3
		"--color-error-100": "245 240 239", // #f5f0ef
		"--color-error-200": "242 236 235", // #f2eceb
		"--color-error-300": "235 225 223", // #ebe1df
		"--color-error-400": "219 203 198", // #dbcbc6
		"--color-error-500": "204 180 174", // #ccb4ae
		"--color-error-600": "184 162 157", // #b8a29d
		"--color-error-700": "153 135 131", // #998783
		"--color-error-800": "122 108 104", // #7a6c68
		"--color-error-900": "100 88 85", // #645855
		// surface | #50a22f 
		"--color-surface-50": "229 241 224", // #e5f1e0
		"--color-surface-100": "220 236 213", // #dcecd5
		"--color-surface-200": "211 232 203", // #d3e8cb
		"--color-surface-300": "185 218 172", // #b9daac
		"--color-surface-400": "133 190 109", // #85be6d
		"--color-surface-500": "80 162 47", // #50a22f
		"--color-surface-600": "72 146 42", // #48922a
		"--color-surface-700": "60 122 35", // #3c7a23
		"--color-surface-800": "48 97 28", // #30611c
		"--color-surface-900": "39 79 23", // #274f17
		
	}
}